use std::path::PathBuf;
use std::sync::Mutex;

use oneiros_db::Database;

pub struct ServiceState {
    pub(crate) database: Mutex<Database>,
    pub(crate) data_dir: PathBuf,
}

impl ServiceState {
    pub fn new(database: Database, data_dir: PathBuf) -> Self {
        Self {
            database: Mutex::new(database),
            data_dir,
        }
    }
}
