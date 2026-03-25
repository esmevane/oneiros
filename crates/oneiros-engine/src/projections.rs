use crate::*;

#[derive(Clone)]
pub struct Projections {
    frames: Vec<Frames>,
}

impl Projections {
    pub fn new(frames: &[Frames]) -> Self {
        Self {
            frames: frames.to_vec(),
        }
    }

    pub fn project() -> Self {
        Self {
            frames: vec![
                Frames::new(&[
                    Frame::new(LevelProjections.all()),
                    Frame::new(TextureProjections.all()),
                    Frame::new(SensationProjections.all()),
                    Frame::new(NatureProjections.all()),
                    Frame::new(PersonaProjections.all()),
                    Frame::new(UrgeProjections.all()),
                ]),
                Frames::new(&[
                    Frame::new(AgentProjections.all()),
                    Frame::new(CognitionProjections.all()),
                    Frame::new(MemoryProjections.all()),
                    Frame::new(ExperienceProjections.all()),
                    Frame::new(ConnectionProjections.all()),
                    Frame::new(StorageProjections.all()),
                ]),
                Frames::new(&[
                    Frame::new(PressureProjections.all()),
                    Frame::new(SearchProjections.all()),
                ]),
            ],
        }
    }

    pub fn system() -> Self {
        Self {
            frames: vec![Frames::new(&[
                Frame::new(TenantProjections.all()),
                Frame::new(ActorProjections.all()),
                Frame::new(BrainProjections.all()),
                Frame::new(TicketProjections.all()),
            ])],
        }
    }

    /// Run all projection migrations.
    pub fn migrate(&self, db: &rusqlite::Connection) -> Result<(), EventError> {
        for frame_set in &self.frames {
            for frame_item in &frame_set.contents {
                for projection in &frame_item.projections {
                    (projection.migrate)(db)?;
                }
            }
        }

        Ok(())
    }

    /// Apply a single event through all frames in order.
    pub fn apply(&self, db: &rusqlite::Connection, event: &StoredEvent) -> Result<(), EventError> {
        for frame_set in &self.frames {
            for frame_item in &frame_set.contents {
                for projection in &frame_item.projections {
                    (projection.apply)(db, event)?;
                }
            }
        }

        Ok(())
    }

    /// Reset all projections across all frames.
    pub fn reset(&self, db: &rusqlite::Connection) -> Result<(), EventError> {
        for frame_set in self.frames.iter().rev() {
            for frame_item in &frame_set.contents {
                for projection in &frame_item.projections {
                    (projection.reset)(db)?;
                }
            }
        }

        Ok(())
    }

    /// Replay a set of events through all frames (for import/rebuild).
    pub fn replay(&self, db: &rusqlite::Connection) -> Result<usize, EventError> {
        let events = EventLog::new(db).load_all()?;

        self.reset(db)?;

        for event in &events {
            self.apply(db, event)?;
        }

        Ok(events.len())
    }
}
