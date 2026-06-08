use rusqlite::params;

use crate::*;

/// Trail projection store — the events ↔ entities bridge.
///
/// Each row joins one event with one entity it touched. Multiple rows per
/// entity are expected (an entity is touched by many events). Multiple rows
/// per event are allowed by the schema, even though current derivation rules
/// usually emit 1:1 at creation time.
///
/// The primary key (`event_id`, `ref`) makes replay idempotent on its own —
/// a second `apply` of the same event lands on the existing row.
pub(crate) struct TrailStore<'a> {
    conn: &'a rusqlite::Connection,
}

impl<'a> TrailStore<'a> {
    pub(crate) fn new(conn: &'a rusqlite::Connection) -> Self {
        Self { conn }
    }

    pub(crate) fn migrate(&self) -> Result<(), EventError> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS trail (
                event_id TEXT NOT NULL,
                ref TEXT NOT NULL,
                event_type TEXT NOT NULL,
                created_at TEXT NOT NULL DEFAULT '',
                PRIMARY KEY (event_id, ref)
            );
             CREATE INDEX IF NOT EXISTS trail_ref_created_at
                ON trail (ref, created_at);
             CREATE INDEX IF NOT EXISTS trail_event_id
                ON trail (event_id);",
        )?;
        Ok(())
    }

    pub(crate) fn reset(&self) -> Result<(), EventError> {
        self.conn.execute("DELETE FROM trail", [])?;
        Ok(())
    }

    /// Apply a single stored event: derive the refs it touches and insert a
    /// row per ref. Replay-safe via `INSERT OR REPLACE` on the composite
    /// primary key.
    pub(crate) fn handle(&self, event: &StoredEvent) -> Result<(), EventError> {
        let Event::Known(known) = &event.data else {
            return Ok(());
        };

        let event_type = known.event_type();
        let refs = derive_refs(known);

        for entity_ref in refs {
            let ref_token = RefToken::new(entity_ref).to_string();
            self.conn.execute(
                "INSERT OR REPLACE INTO trail (event_id, ref, event_type, created_at)
                 VALUES (?1, ?2, ?3, ?4)",
                params![
                    event.id.to_string(),
                    ref_token,
                    event_type,
                    event.created_at.as_string(),
                ],
            )?;
        }
        Ok(())
    }
}

/// Pure derivation rule: given an event, return the refs of the entities it
/// touched. Single source of truth for what the trail projection records.
///
/// Policy:
/// - Entity-creating/mutating/removing events emit the touched entity's ref.
/// - Cognition events emit ONLY the cognition's ref — the texture and the
///   agent live on the cognition itself and are reached through that, not
///   through trail. Trail is for event ↔ entity, not entity ↔ entity.
/// - Events keyed by name (AgentRemoved, BookmarkSwitched, continuity
///   events) emit nothing — Ref takes typed IDs, and we would need a
///   name→id resolution at projection time that doesn't belong here. When
///   in doubt, emit nothing rather than over-couple.
fn derive_refs(event: &Events) -> Vec<Ref> {
    match event {
        // Vocabulary — set carries the entity (named), remove carries the name.
        Events::Level(LevelEvents::LevelSet(set)) => {
            one(set.clone().current(), |v| Ref::level(v.level.name))
        }
        Events::Level(LevelEvents::LevelRemoved(removed)) => {
            one(removed.clone().current(), |v| Ref::level(v.name))
        }
        Events::Texture(TextureEvents::TextureSet(set)) => {
            one(set.clone().current(), |v| Ref::texture(v.texture.name))
        }
        Events::Texture(TextureEvents::TextureRemoved(removed)) => {
            one(removed.clone().current(), |v| Ref::texture(v.name))
        }
        Events::Sensation(SensationEvents::SensationSet(set)) => {
            one(set.clone().current(), |v| Ref::sensation(v.sensation.name))
        }
        Events::Sensation(SensationEvents::SensationRemoved(removed)) => {
            one(removed.clone().current(), |v| Ref::sensation(v.name))
        }
        Events::Nature(NatureEvents::NatureSet(set)) => {
            one(set.clone().current(), |v| Ref::nature(v.nature.name))
        }
        Events::Nature(NatureEvents::NatureRemoved(removed)) => {
            one(removed.clone().current(), |v| Ref::nature(v.name))
        }
        Events::Persona(PersonaEvents::PersonaSet(set)) => {
            one(set.clone().current(), |v| Ref::persona(v.persona.name))
        }
        Events::Persona(PersonaEvents::PersonaRemoved(removed)) => {
            one(removed.clone().current(), |v| Ref::persona(v.name))
        }
        Events::Urge(UrgeEvents::UrgeSet(set)) => {
            one(set.clone().current(), |v| Ref::urge(v.urge.name))
        }
        Events::Urge(UrgeEvents::UrgeRemoved(removed)) => {
            one(removed.clone().current(), |v| Ref::urge(v.name))
        }

        // Agents — name-keyed remove doesn't emit (no name→id resolution at
        // projection time).
        Events::Agent(AgentEvents::AgentCreated(created)) => {
            one(created.clone().current(), |v| Ref::agent(v.agent.id))
        }
        Events::Agent(AgentEvents::AgentUpdated(updated)) => {
            one(updated.clone().current(), |v| Ref::agent(v.agent.id))
        }
        Events::Agent(AgentEvents::AgentRemoved(_)) => Vec::new(),

        // Cognition/memory/experience — content-bearing entities.
        Events::Cognition(CognitionEvents::CognitionAdded(added)) => {
            one(added.clone().current(), |v| Ref::cognition(v.cognition.id))
        }
        Events::Memory(MemoryEvents::MemoryAdded(added)) => {
            one(added.clone().current(), |v| Ref::memory(v.memory.id))
        }
        Events::Experience(ExperienceEvents::ExperienceCreated(created)) => {
            one(created.clone().current(), |v| {
                Ref::experience(v.experience.id)
            })
        }
        Events::Experience(ExperienceEvents::ExperienceDescriptionUpdated(updated)) => {
            one(updated.clone().current(), |v| Ref::experience(v.id))
        }
        Events::Experience(ExperienceEvents::ExperienceSensationUpdated(updated)) => {
            one(updated.clone().current(), |v| Ref::experience(v.id))
        }

        // Connections and storage.
        Events::Connection(ConnectionEvents::ConnectionCreated(created)) => {
            one(created.clone().current(), |v| {
                Ref::connection(v.connection.id)
            })
        }
        Events::Connection(ConnectionEvents::ConnectionRemoved(removed)) => {
            one(removed.clone().current(), |v| Ref::connection(v.id))
        }
        Events::Storage(StorageEvents::StorageSet(set)) => {
            one(set.clone().current(), |v| Ref::storage(v.entry.key))
        }
        Events::Storage(StorageEvents::StorageRemoved(removed)) => {
            one(removed.clone().current(), |v| Ref::storage(v.key))
        }

        // Continuity — keyed by agent name. No emission (see policy above).
        Events::Continuity(_) => Vec::new(),

        // Host-tier entities.
        Events::Tenant(TenantEvents::TenantCreated(created)) => {
            one(created.clone().current(), |v| Ref::tenant(v.tenant.id))
        }
        Events::Actor(ActorEvents::ActorCreated(created)) => {
            one(created.clone().current(), |v| Ref::actor(v.actor.id))
        }
        Events::Project(ProjectEvents::ProjectCreated(created)) => {
            one(created.clone().current(), |v| Ref::project(v.project.id))
        }
        Events::Ticket(TicketEvents::TicketIssued(issued)) => {
            one(issued.clone().current(), |v| Ref::ticket(v.ticket.id))
        }
        Events::Ticket(TicketEvents::TicketUsed(used)) => {
            one(used.clone().current(), |v| Ref::ticket(v.ticket_id))
        }
        Events::Ticket(TicketEvents::TicketRejected(rejected)) => rejected
            .clone()
            .current()
            .ok()
            .and_then(|v| v.ticket_id.map(|id| vec![Ref::ticket(id)]))
            .unwrap_or_default(),

        // Bookmarks — id-keyed lifecycle events emit, name-keyed don't.
        Events::Bookmark(BookmarkEvents::BookmarkCreated(created)) => {
            one(created.clone().current(), |v| Ref::bookmark(v.bookmark.id))
        }
        Events::Bookmark(BookmarkEvents::BookmarkForked(forked)) => {
            one(forked.clone().current(), |v| Ref::bookmark(v.bookmark.id))
        }
        Events::Bookmark(BookmarkEvents::BookmarkSwitched(_))
        | Events::Bookmark(BookmarkEvents::BookmarkMerged(_))
        | Events::Bookmark(BookmarkEvents::BookmarkShared(_)) => Vec::new(),
        Events::Bookmark(BookmarkEvents::BookmarkFollowed(followed)) => {
            one(followed.clone().current(), |v| Ref::follow(v.id))
        }
        Events::Bookmark(BookmarkEvents::BookmarkCollected(collected)) => {
            one(collected.clone().current(), |v| Ref::follow(v.follow_id))
        }
        Events::Bookmark(BookmarkEvents::BookmarkUnfollowed(unfollowed)) => {
            one(unfollowed.clone().current(), |v| Ref::follow(v.follow_id))
        }

        // Slices — metadata events, no entity refs to emit.
        Events::Slice(_) => Vec::new(),

        // Peers.
        Events::Peer(PeerEvents::PeerAdded(added)) => {
            one(added.clone().current(), |v| Ref::peer(v.id))
        }
        Events::Peer(PeerEvents::PeerUpdated(updated)) => {
            one(updated.clone().current(), |v| Ref::peer(v.id))
        }
        Events::Peer(PeerEvents::PeerRemoved(removed)) => {
            one(removed.clone().current(), |v| Ref::peer(v.id))
        }

        // Remotes.
        Events::Remote(RemoteEvents::RemoteAdded(added)) => {
            one(added.clone().current(), |v| Ref::remote(v.remote.id))
        }
        Events::Remote(RemoteEvents::RemoteRemoved(removed)) => {
            one(removed.clone().current(), |v| Ref::remote(v.id))
        }

        // Ticket revocation.
        Events::Ticket(TicketEvents::TicketRevoked(revoked)) => {
            one(revoked.clone().current(), |v| Ref::ticket(v.ticket_id))
        }
    }
}

/// Lift a versioned event to its current variant and map it to a single ref.
/// Returns an empty vec when upcasting fails — the event still entered the
/// log, but its shape can't be honored, so we don't fabricate a ref.
fn one<V, F: FnOnce(V) -> Ref>(versioned: Result<V, UpcastError>, to_ref: F) -> Vec<Ref> {
    versioned.ok().map(|v| vec![to_ref(v)]).unwrap_or_default()
}
