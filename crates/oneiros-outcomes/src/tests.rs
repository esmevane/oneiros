use std::sync::{Arc, Mutex};

use tracing_subscriber::fmt::MakeWriter;

use crate::{Outcomes, PromptReporter, Reportable, Reporter};

#[derive(Clone)]
struct CaptureWriter(Arc<Mutex<Vec<u8>>>);

impl std::io::Write for CaptureWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.lock().unwrap().extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl<'a> MakeWriter<'a> for CaptureWriter {
    type Writer = CaptureWriter;

    fn make_writer(&'a self) -> Self::Writer {
        self.clone()
    }
}

fn capture_subscriber(writer: CaptureWriter) -> impl tracing::Subscriber {
    tracing_subscriber::fmt()
        .with_writer(writer)
        .with_ansi(false)
        .with_target(false)
        .with_max_level(tracing::Level::TRACE)
        .finish()
}

fn captured_output(buffer: &Arc<Mutex<Vec<u8>>>) -> String {
    String::from_utf8(buffer.lock().unwrap().clone()).unwrap()
}

// A minimal Reportable implementation for testing.
enum TestOutcome {
    Info(String),
    Warn(String),
    Debug(String),
}

impl Reportable for TestOutcome {
    fn level(&self) -> tracing::Level {
        match self {
            TestOutcome::Info(_) => tracing::Level::INFO,
            TestOutcome::Warn(_) => tracing::Level::WARN,
            TestOutcome::Debug(_) => tracing::Level::DEBUG,
        }
    }

    fn message(&self) -> String {
        match self {
            TestOutcome::Info(msg) | TestOutcome::Warn(msg) | TestOutcome::Debug(msg) => {
                msg.clone()
            }
        }
    }
}

// Wrapping type for testing map_into.
enum WrappedOutcome {
    #[expect(unused, reason = "This is ok to remove if we start using it")]
    Test(TestOutcome),
}

impl From<TestOutcome> for WrappedOutcome {
    fn from(outcome: TestOutcome) -> Self {
        WrappedOutcome::Test(outcome)
    }
}

#[test]
fn emit_captures_caller_location() {
    let buffer = Arc::new(Mutex::new(Vec::new()));
    let subscriber = capture_subscriber(CaptureWriter(buffer.clone()));

    tracing::subscriber::with_default(subscriber, || {
        let mut outcomes = Outcomes::new();
        outcomes.emit(TestOutcome::Info("location test".into()));
    });

    let output = captured_output(&buffer);
    let this_file = file!();

    assert!(
        output.contains(this_file),
        "Expected caller file '{this_file}' in output, got:\n{output}"
    );
    assert!(
        output.contains("location test"),
        "Expected message in output, got:\n{output}"
    );
}

#[test]
fn emit_fires_at_correct_level() {
    let buffer = Arc::new(Mutex::new(Vec::new()));
    let subscriber = capture_subscriber(CaptureWriter(buffer.clone()));

    tracing::subscriber::with_default(subscriber, || {
        let mut outcomes = Outcomes::new();
        outcomes.emit(TestOutcome::Warn("warn test".into()));
        outcomes.emit(TestOutcome::Debug("debug test".into()));
    });

    let output = captured_output(&buffer);

    assert!(
        output.contains("WARN"),
        "Expected WARN in output, got:\n{output}"
    );
    assert!(
        output.contains("DEBUG"),
        "Expected DEBUG in output, got:\n{output}"
    );
}

#[test]
fn emit_collects_outcomes() {
    let mut outcomes = Outcomes::new();

    assert!(outcomes.is_empty());
    assert_eq!(outcomes.len(), 0);

    outcomes.emit(TestOutcome::Info("first".into()));
    outcomes.emit(TestOutcome::Warn("second".into()));

    assert!(!outcomes.is_empty());
    assert_eq!(outcomes.len(), 2);
}

#[test]
fn iter_yields_references() {
    let mut outcomes = Outcomes::new();
    outcomes.emit(TestOutcome::Info("a".into()));
    outcomes.emit(TestOutcome::Warn("b".into()));

    let messages: Vec<String> = outcomes.iter().map(|o| o.message()).collect();
    assert_eq!(messages, vec!["a", "b"]);
}

#[test]
fn into_iter_consumes() {
    let mut outcomes = Outcomes::new();
    outcomes.emit(TestOutcome::Info("a".into()));
    outcomes.emit(TestOutcome::Warn("b".into()));

    let collected: Vec<TestOutcome> = outcomes.into_iter().collect();
    assert_eq!(collected.len(), 2);
}

#[test]
fn map_into_converts_without_re_emitting() {
    let buffer = Arc::new(Mutex::new(Vec::new()));
    let subscriber = capture_subscriber(CaptureWriter(buffer.clone()));

    let wrapped = tracing::subscriber::with_default(subscriber, || {
        let mut outcomes: Outcomes<TestOutcome> = Outcomes::new();
        outcomes.emit(TestOutcome::Info("mapped".into()));

        // Clear the buffer so we can check that map_into doesn't emit
        buffer.lock().unwrap().clear();

        outcomes.map_into::<WrappedOutcome>()
    });

    let output = captured_output(&buffer);

    // map_into should NOT have emitted anything
    assert!(
        output.is_empty(),
        "map_into should not emit, but got:\n{output}"
    );

    // But the outcomes should still be there
    assert_eq!(wrapped.len(), 1);
}

#[test]
fn into_inner_returns_vec() {
    let mut outcomes = Outcomes::new();
    outcomes.emit(TestOutcome::Info("inner".into()));

    let vec = outcomes.into_inner();
    assert_eq!(vec.len(), 1);
}

// -- log_message / prompt / Reporter tests --

// An outcome with separate log and prompt messages.
#[expect(
    dead_code,
    reason = "variants exercised through Reportable trait, not pattern matched"
)]
enum DetailedOutcome {
    WithLog(String),
    WithPrompt(String),
    WithBoth(String),
    Plain(String),
}

impl Reportable for DetailedOutcome {
    fn level(&self) -> tracing::Level {
        tracing::Level::INFO
    }

    fn message(&self) -> String {
        match self {
            Self::WithLog(m) | Self::WithPrompt(m) | Self::WithBoth(m) | Self::Plain(m) => {
                m.clone()
            }
        }
    }

    fn log_message(&self) -> String {
        match self {
            Self::WithLog(_) => "internal log detail".into(),
            Self::WithBoth(_) => "internal log for both".into(),
            _ => self.message(),
        }
    }

    fn prompt(&self) -> Option<String> {
        match self {
            Self::WithPrompt(_) => Some("try this next".into()),
            Self::WithBoth(_) => Some("do something".into()),
            _ => None,
        }
    }
}

#[test]
fn log_message_defaults_to_message() {
    let outcome = TestOutcome::Info("hello".into());
    assert_eq!(outcome.log_message(), "hello");
}

#[test]
fn log_message_can_differ_from_message() {
    let outcome = DetailedOutcome::WithLog("user sees this".into());
    assert_eq!(outcome.message(), "user sees this");
    assert_eq!(outcome.log_message(), "internal log detail");
}

#[test]
fn prompt_defaults_to_none() {
    let outcome = TestOutcome::Info("hello".into());
    assert_eq!(outcome.prompt(), None);
}

#[test]
fn prompt_returns_some_when_provided() {
    let outcome = DetailedOutcome::WithPrompt("msg".into());
    assert_eq!(outcome.prompt(), Some("try this next".into()));
}

#[test]
fn emit_uses_log_message_for_tracing() {
    let buffer = Arc::new(Mutex::new(Vec::new()));
    let subscriber = capture_subscriber(CaptureWriter(buffer.clone()));

    tracing::subscriber::with_default(subscriber, || {
        let mut outcomes = Outcomes::new();
        outcomes.emit(DetailedOutcome::WithLog("user msg".into()));
    });

    let output = captured_output(&buffer);

    assert!(
        output.contains("internal log detail"),
        "Expected log_message in tracing output, got:\n{output}"
    );
    assert!(
        !output.contains("user msg"),
        "Should not contain user message in tracing output, got:\n{output}"
    );
}

#[test]
fn console_reporter_prints_message() {
    // PromptReporter uses println, so we just verify it doesn't panic
    // and the trait is object-safe.
    let reporter: &dyn Reporter = &PromptReporter;
    let outcome = TestOutcome::Info("reporter test".into());
    reporter.report(&outcome);
}

// Verify log.* fields are still swallowed (regression guard).
#[test]
fn log_dot_fields_are_swallowed() {
    let buffer = Arc::new(Mutex::new(Vec::new()));
    let subscriber = capture_subscriber(CaptureWriter(buffer.clone()));

    tracing::subscriber::with_default(subscriber, || {
        tracing::info!(log.file = "should_not_appear.rs", "hello");
    });

    let output = captured_output(&buffer);

    assert!(
        !output.contains("should_not_appear"),
        "log.* fields should be suppressed by tracing-subscriber, got:\n{output}"
    );
}
