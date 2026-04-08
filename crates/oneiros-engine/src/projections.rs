use crate::*;

#[derive(Clone)]
pub struct Projections<T> {
    frames: Vec<Frames>,
    reducers: ReducerPipeline<T>,
    canon: Canon,
}

impl<T: Clone + Default + Materialize> Projections<T> {
    pub fn new(frames: &[Frames], reducers: ReducerPipeline<T>) -> Self {
        Self {
            frames: frames.to_vec(),
            reducers,
            canon: Canon::new(),
        }
    }

    pub fn with_canon(frames: &[Frames], reducers: ReducerPipeline<T>, canon: Canon) -> Self {
        Self {
            frames: frames.to_vec(),
            reducers,
            canon,
        }
    }

    /// The underlying CRDT document.
    pub fn canon(&self) -> &Canon {
        &self.canon
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

        self.reducers.apply(&event.data)?;
        self.canon.reconcile(&self.reducers.state()?)?;

        Ok(())
    }

    /// Apply a single event through SQLite frame projections only.
    /// Skips the reducer and canon — used during bookmark switch
    /// when the canon is already correct and only SQLite needs rebuilding.
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
        self.canon.reset()?;

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

impl Projections<BrainCanon> {
    pub fn project() -> Self {
        Self::project_with_canon(Canon::new())
    }

    pub fn project_with_canon(canon: Canon) -> Self {
        Self::project_with_entry(canon, ReducerPipeline::brain())
    }

    pub fn project_with_entry(canon: Canon, pipeline: ReducerPipeline<BrainCanon>) -> Self {
        Self::with_canon(
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
            canon,
        )
    }
}

impl Projections<SystemCanon> {
    pub fn system() -> Self {
        Self::system_with_canon(Canon::new())
    }

    pub fn system_with_canon(canon: Canon) -> Self {
        Self::with_canon(
            &[Frames::new(&[
                Frame::new(TenantProjections.all()),
                Frame::new(ActorProjections.all()),
                Frame::new(BrainProjections.all()),
                Frame::new(TicketProjections.all()),
                Frame::new(BookmarkProjections.all()),
            ])],
            ReducerPipeline::system(),
            canon,
        )
    }
}
