use tokio::sync::{mpsc, oneshot};

/// An actor — owns state, handles messages, produces responses.
///
/// Actors run on their own task. They receive messages through a channel
/// and process them sequentially. This means:
/// - The actor's state can be `!Send` (e.g., rusqlite Database)
/// - No Mutex needed — the mailbox serializes access
/// - The actor decides its own async strategy
pub trait Actor: Send + 'static {
    /// The message type this actor handles.
    type Message: Send + 'static;

    /// The response type this actor produces.
    type Response: Send + 'static;

    /// Handle a message and produce a response.
    fn handle(
        &mut self,
        message: Self::Message,
    ) -> impl std::future::Future<Output = Self::Response> + Send;
}

/// A handle to a running actor — the send side of its mailbox.
///
/// Handles are `Clone + Send` — they can be shared across tasks,
/// passed to HTTP handlers, held by other actors. The actor itself
/// may be `!Send` (it never leaves its task); the handle is always `Send`.
pub struct Handle<M: Send + 'static, R: Send + 'static> {
    tx: mpsc::Sender<Envelope<M, R>>,
}

// Manual Clone — mpsc::Sender is Clone regardless of M/R's Clone status
impl<M: Send + 'static, R: Send + 'static> Clone for Handle<M, R> {
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
        }
    }
}

/// An envelope: a message paired with a reply channel.
struct Envelope<M, R> {
    message: M,
    reply: oneshot::Sender<R>,
}

impl<M: Send + 'static, R: Send + 'static> Handle<M, R> {
    /// Send a message and await the response.
    pub async fn send(&self, message: M) -> Result<R, SendError> {
        let (reply_tx, reply_rx) = oneshot::channel();
        self.tx
            .send(Envelope {
                message,
                reply: reply_tx,
            })
            .await
            .map_err(|_| SendError::ActorStopped)?;

        reply_rx.await.map_err(|_| SendError::NoResponse)
    }
}

/// Spawn an actor on a new tokio task, returning a handle to its mailbox.
///
/// The actor runs a loop: receive envelope, call handle(), send reply.
/// The loop ends when all handles are dropped (the channel closes).
pub fn spawn<A: Actor>(mut actor: A) -> Handle<A::Message, A::Response> {
    let (tx, mut rx) = mpsc::channel::<Envelope<A::Message, A::Response>>(32);

    tokio::spawn(async move {
        while let Some(envelope) = rx.recv().await {
            let response = actor.handle(envelope.message).await;
            // If the caller dropped the reply channel, that's fine — discard the response
            let _ = envelope.reply.send(response);
        }
    });

    Handle { tx }
}

/// A synchronous actor — owns `!Send` state, runs on a dedicated thread.
///
/// Unlike `Actor` (which is `Send` and runs on a tokio task), `SyncActor`
/// can own `!Send` types (like rusqlite's `Database`). It runs on its own
/// OS thread with a blocking receive loop. The `Handle` is identical —
/// callers can't tell the difference.
///
/// This is the solution to the `!Send` problem: the database lives on
/// its own thread, all access is serialized through the mailbox, and the
/// `Handle` is `Clone + Send` for use in async contexts.
pub trait SyncActor: 'static {
    // Note: no Send bound — the actor stays on its thread
    type Message: Send + 'static;
    type Response: Send + 'static;

    fn handle(&mut self, message: Self::Message) -> Self::Response;
}

/// Spawn a synchronous actor on a dedicated OS thread.
///
/// Because the actor may be `!Send`, it can't cross the thread boundary.
/// Instead, the caller provides a factory function that constructs the
/// actor on the dedicated thread. The factory's inputs must be `Send`.
///
/// The actor runs a blocking loop: receive envelope, call handle(), send reply.
/// The loop ends when all handles are dropped.
pub fn spawn_sync<A, F>(factory: F) -> Handle<A::Message, A::Response>
where
    A: SyncActor,
    F: FnOnce() -> A + Send + 'static,
{
    let (tx, mut rx) = mpsc::channel::<Envelope<A::Message, A::Response>>(32);

    std::thread::spawn(move || {
        let mut actor = factory();

        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("build runtime for sync actor");

        rt.block_on(async move {
            while let Some(envelope) = rx.recv().await {
                let response = actor.handle(envelope.message);
                let _ = envelope.reply.send(response);
            }
        });
    });

    Handle { tx }
}

/// Errors when sending a message to an actor.
#[derive(Debug, thiserror::Error)]
pub enum SendError {
    #[error("Actor has stopped")]
    ActorStopped,

    #[error("Actor did not respond")]
    NoResponse,
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Counter {
        count: u32,
    }

    enum CounterMessage {
        Increment,
        Get,
    }

    impl Actor for Counter {
        type Message = CounterMessage;
        type Response = u32;

        async fn handle(&mut self, message: CounterMessage) -> u32 {
            match message {
                CounterMessage::Increment => {
                    self.count += 1;
                    self.count
                }
                CounterMessage::Get => self.count,
            }
        }
    }

    #[tokio::test]
    async fn actor_handles_messages() {
        let handle = spawn(Counter { count: 0 });

        let result = handle.send(CounterMessage::Increment).await.unwrap();
        assert_eq!(result, 1);

        let result = handle.send(CounterMessage::Increment).await.unwrap();
        assert_eq!(result, 2);

        let result = handle.send(CounterMessage::Get).await.unwrap();
        assert_eq!(result, 2);
    }

    #[tokio::test]
    async fn handle_is_clone_and_send() {
        let handle = spawn(Counter { count: 0 });
        let handle2 = handle.clone();

        handle.send(CounterMessage::Increment).await.unwrap();
        let result = handle2.send(CounterMessage::Get).await.unwrap();
        assert_eq!(result, 1);
    }

    #[tokio::test]
    async fn actor_stops_when_handles_dropped() {
        let handle = spawn(Counter { count: 0 });
        handle.send(CounterMessage::Increment).await.unwrap();

        // Drop the handle — the actor's loop should end
        drop(handle);

        // Can't send anymore — but we can't test this without another handle
        // The actor just quietly stops. This test verifies no panic on drop.
    }

    // ── SyncActor tests ─────────────────────────────────────────────

    /// A !Send type — simulates rusqlite's Database
    struct NotSendState {
        value: std::rc::Rc<std::cell::Cell<u32>>,
    }

    enum NotSendMessage {
        Set(u32),
        Get,
    }

    impl SyncActor for NotSendState {
        type Message = NotSendMessage;
        type Response = u32;

        fn handle(&mut self, message: NotSendMessage) -> u32 {
            match message {
                NotSendMessage::Set(v) => {
                    self.value.set(v);
                    v
                }
                NotSendMessage::Get => self.value.get(),
            }
        }
    }

    #[tokio::test]
    async fn sync_actor_handles_not_send_state() {
        // The !Send state is constructed ON the actor's thread via factory
        let handle = spawn_sync(|| NotSendState {
            value: std::rc::Rc::new(std::cell::Cell::new(0)),
        });

        let result = handle.send(NotSendMessage::Set(42)).await.unwrap();
        assert_eq!(result, 42);

        let result = handle.send(NotSendMessage::Get).await.unwrap();
        assert_eq!(result, 42);
    }

    #[tokio::test]
    async fn sync_actor_handle_is_send_and_clone() {
        let handle = spawn_sync(|| NotSendState {
            value: std::rc::Rc::new(std::cell::Cell::new(0)),
        });
        let handle2 = handle.clone();

        handle.send(NotSendMessage::Set(99)).await.unwrap();
        let result = handle2.send(NotSendMessage::Get).await.unwrap();
        assert_eq!(result, 99);
    }
}
