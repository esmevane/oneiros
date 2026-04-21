use crate::*;

#[derive(Clone)]
pub struct Projections<T> {
    frames: Vec<Frames>,
    reducers: ReducerPipeline<T>,
}

impl<T: Clone + Default> Projections<T> {
    pub fn new(frames: &[Frames], reducers: ReducerPipeline<T>) -> Self {
        Self {
            frames: frames.to_vec(),
            reducers,
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
    #[tracing::instrument(skip_all, fields(event_type = event.data.event_type(), sequence = event.sequence), err(Display))]
    pub fn apply(&self, db: &rusqlite::Connection, event: &StoredEvent) -> Result<(), EventError> {
        for frame_set in &self.frames {
            for frame_item in &frame_set.contents {
                for projection in &frame_item.projections {
                    (projection.apply)(db, event)?;
                }
            }
        }

        self.reducers.apply(&event.data)?;

        Ok(())
    }

    /// Apply a single event through SQLite frame projections only.
    /// Skips the reducer — used during bookmark switch when only
    /// SQLite needs rebuilding.
    pub fn apply_frames(
        &self,
        db: &rusqlite::Connection,
        event: &StoredEvent,
    ) -> Result<(), EventError> {
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

        self.reducers.reset()?;

        Ok(())
    }

    /// Replay all events through frames and reducers.
    #[tracing::instrument(skip_all, err(Display))]
    pub fn replay(&self, db: &rusqlite::Connection, log: &EventLog) -> Result<usize, EventError> {
        let events = log.load_all()?;

        self.reset(db)?;

        db.execute_batch("BEGIN")?;

        let result = (|| -> Result<(), EventError> {
            for event in &events {
                self.apply_frames(db, event)?;
                self.reducers.apply(&event.data)?;
            }
            Ok(())
        })();

        match result {
            Ok(()) => db.execute_batch("COMMIT")?,
            Err(e) => {
                let _ = db.execute_batch("ROLLBACK");
                return Err(e);
            }
        }

        tracing::info!(replayed = events.len(), "projections replay complete");
        Ok(events.len())
    }
}

impl Projections<BrainCanon> {
    /// Apply a single event — projections, reducer, then sync
    /// reducer-computed pressures to SQLite.
    pub fn apply_brain(
        &self,
        db: &rusqlite::Connection,
        event: &StoredEvent,
    ) -> Result<(), EventError> {
        self.apply(db, event)?;
        self.sync_pressures(db)?;
        Ok(())
    }

    /// Write reducer-computed pressure state to the SQLite table.
    fn sync_pressures(&self, db: &rusqlite::Connection) -> Result<(), EventError> {
        let state = self.reducers.state()?;
        let store = PressureStore::new(db);

        for pressure in state.pressures.values() {
            store.upsert(
                &pressure.agent_id,
                &pressure.urge,
                &pressure.data,
                &pressure.updated_at,
            )?;
        }

        Ok(())
    }

    /// Replay for brain projections — includes pressure sync at the end.
    pub fn replay_brain(
        &self,
        db: &rusqlite::Connection,
        log: &EventLog,
    ) -> Result<usize, EventError> {
        let count = self.replay(db, log)?;
        self.sync_pressures(db)?;
        Ok(count)
    }

    pub fn project() -> Self {
        Self::project_with_pipeline(ReducerPipeline::brain())
    }

    pub fn project_with_pipeline(pipeline: ReducerPipeline<BrainCanon>) -> Self {
        Self::new(
            &[
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
            pipeline,
        )
    }
}

impl Projections<SystemCanon> {
    pub fn system() -> Self {
        Self::new(
            &[Frames::new(&[
                Frame::new(TenantProjections.all()),
                Frame::new(ActorProjections.all()),
                Frame::new(BrainProjections.all()),
                Frame::new(TicketProjections.all()),
                Frame::new(BookmarkProjections.all()),
                Frame::new(PeerProjections.all()),
                Frame::new(FollowProjections.all()),
            ])],
            ReducerPipeline::system(),
        )
    }
}
