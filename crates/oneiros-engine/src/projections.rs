use crate::*;

#[derive(Clone)]
pub(crate) struct Projections<T> {
    frames: Vec<Frames>,
    reducers: ReducerPipeline<T>,
}

impl<T: Clone + Default> Projections<T> {
    pub(crate) fn new(frames: &[Frames], reducers: ReducerPipeline<T>) -> Self {
        Self {
            frames: frames.to_vec(),
            reducers,
        }
    }

    /// Run all projection migrations.
    pub(crate) fn migrate(&self, db: &rusqlite::Connection) -> Result<(), EventError> {
        for frame_set in &self.frames {
            for frame_item in &frame_set.contents {
                for projection in &frame_item.projections {
                    let _span = tracing::trace_span!("projection.migrate", name = projection.name)
                        .entered();
                    (projection.migrate)(db)?;
                }
            }
        }

        Ok(())
    }

    /// Apply a single event through all frames in order.
    #[tracing::instrument(skip_all, fields(event_type = event.data.event_type(), sequence = event.sequence), err(Display))]
    pub(crate) fn apply(
        &self,
        db: &rusqlite::Connection,
        event: &StoredEvent,
    ) -> Result<(), EventError> {
        for frame_set in &self.frames {
            for frame_item in &frame_set.contents {
                for projection in &frame_item.projections {
                    let _span =
                        tracing::trace_span!("projection.apply", name = projection.name).entered();
                    (projection.apply)(db, event)?;
                }
            }
        }

        if let Event::Known(data) = &event.data {
            self.reducers.apply(data)?;
        }

        Ok(())
    }

    /// Apply a single event through SQLite frame projections only.
    /// Skips the reducer — used during bookmark switch when only
    /// SQLite needs rebuilding.
    pub(crate) fn apply_frames(
        &self,
        db: &rusqlite::Connection,
        event: &StoredEvent,
    ) -> Result<(), EventError> {
        for frame_set in &self.frames {
            for frame_item in &frame_set.contents {
                for projection in &frame_item.projections {
                    let _span =
                        tracing::trace_span!("projection.apply", name = projection.name).entered();
                    (projection.apply)(db, event)?;
                }
            }
        }
        Ok(())
    }

    /// Reset all projections across all frames.
    pub(crate) fn reset(&self, db: &rusqlite::Connection) -> Result<(), EventError> {
        for frame_set in self.frames.iter().rev() {
            for frame_item in &frame_set.contents {
                for projection in &frame_item.projections {
                    let _span =
                        tracing::trace_span!("projection.reset", name = projection.name).entered();
                    (projection.reset)(db)?;
                }
            }
        }

        self.reducers.reset()?;

        Ok(())
    }

    /// Replay all events through frames and reducers.
    #[tracing::instrument(skip_all, err(Display))]
    pub(crate) fn replay(
        &self,
        db: &rusqlite::Connection,
        log: &EventLog,
    ) -> Result<usize, EventError> {
        let events = log.load_all()?;

        self.reset(db)?;

        db.execute_batch("BEGIN")?;

        let result = (|| -> Result<(), EventError> {
            for event in &events {
                self.apply_frames(db, event)?;
                if let Event::Known(data) = &event.data {
                    self.reducers.apply(data)?;
                }
            }
            Ok(())
        })();

        match result {
            Ok(()) => db.execute_batch("commit")?,
            Err(e) => {
                let _ = db.execute_batch("rollback");
                return Err(e);
            }
        }

        tracing::info!(replayed = events.len(), "projections replay complete");
        Ok(events.len())
    }
}

impl Projections<ProjectCanon> {
    /// Apply a single event — projections, reducer, then sync
    /// reducer-computed pressures to SQLite.
    pub(crate) fn apply_project(
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

    /// Replay for project projections — includes pressure sync at the end.
    pub(crate) fn replay_project(
        &self,
        db: &rusqlite::Connection,
        log: &EventLog,
    ) -> Result<usize, EventError> {
        let count = self.replay(db, log)?;
        self.sync_pressures(db)?;
        Ok(count)
    }

    pub(crate) fn project() -> Self {
        Self::project_with_pipeline(ReducerPipeline::project())
    }

    pub(crate) fn project_with_pipeline(pipeline: ReducerPipeline<ProjectCanon>) -> Self {
        Self::new(
            &[
                // Vocabulary: idempotent definitions that other projections
                // reference. These must migrate and apply first so that
                // foreign-key targets exist before records land.
                Frames::new(&[
                    Frame::new(LevelProjections.all()),
                    Frame::new(TextureProjections.all()),
                    Frame::new(SensationProjections.all()),
                    Frame::new(NatureProjections.all()),
                    Frame::new(PersonaProjections.all()),
                    Frame::new(UrgeProjections.all()),
                ]),
                // Records: the core domain entities. Each projection is
                // commutative within its own domain — ordering between
                // domains in this tier is not load-bearing.
                Frames::new(&[
                    Frame::new(AgentProjections.all()),
                    Frame::new(CognitionProjections.all()),
                    Frame::new(MemoryProjections.all()),
                    Frame::new(ExperienceProjections.all()),
                    Frame::new(ConnectionProjections.all()),
                    Frame::new(StorageProjections.all()),
                ]),
                // Aggregations: cross-domain or derived views that read
                // from records. Applied last so the data they aggregate
                // is already materialized.
                Frames::new(&[
                    Frame::new(PressureProjections.all()),
                    Frame::new(SearchProjections.all()),
                    Frame::new(TrailProjections.all()),
                ]),
            ],
            pipeline,
        )
    }
}

impl Projections<HostCanon> {
    pub(crate) fn host() -> Self {
        Self::new(
            &[Frames::new(&[
                Frame::new(TenantProjections.all()),
                Frame::new(ActorProjections.all()),
                Frame::new(ProjectProjections.all()),
                Frame::new(TicketProjections.all()),
                Frame::new(BookmarkProjections.all()),
                Frame::new(SliceProjections.all()),
                Frame::new(PeerProjections.all()),
                Frame::new(FollowProjections.all()),
            ])],
            ReducerPipeline::host(),
        )
    }
}
