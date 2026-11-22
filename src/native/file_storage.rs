//! Persistent storage for the terminal application.

use std::{
    fs, io,
    path::{Path, PathBuf},
};

use directories::ProjectDirs;

use crate::storage::{Storage, StorageError, StorageKey};

#[derive(Debug)]
pub struct FileStorage {
    config_directory: PathBuf,
    data_directory: PathBuf,
}

/// Identifiers passed to [`ProjectDirs`]. They determine where data is stored,
/// so changing any of them makes existing data unreachable.
const QUALIFIER: &str = "com";
const ORGANIZATION: &str = "emilhals";
const APPLICATION: &str = "chess-trainer";

impl FileStorage {
    pub fn new(config_directory: PathBuf, data_directory: PathBuf) -> Self {
        Self {
            config_directory,
            data_directory,
        }
    }

    pub fn with_default_location() -> Result<Self, StorageError> {
        let dirs = ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION)
            .ok_or(StorageError::Unavailable)?;

        Ok(Self::new(
            dirs.config_dir().to_path_buf(),
            dirs.data_dir().to_path_buf(),
        ))
    }

    fn path_for(&self, key: StorageKey) -> PathBuf {
        let directory = match key {
            StorageKey::PlayerData => &self.data_directory,
            StorageKey::Settings => &self.config_directory,
        };

        directory.join(format!("{}.json", key.name()))
    }
}

impl Storage for FileStorage {
    fn load(&self, key: StorageKey) -> Result<Option<String>, StorageError> {
        let path = self.path_for(key);
        let content = fs::read_to_string(&path);

        match content {
            Ok(text) => Ok(Some(text)),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(io_failure("read", &path, err)),
        }
    }

    fn save(&mut self, key: StorageKey, data: &str) -> Result<(), StorageError> {
        let path = self.path_for(key);

        if let Some(folder) = path.parent() {
            fs::create_dir_all(folder).map_err(|err| io_failure("create folder", folder, err))?;
        }

        let temp_path = path.with_extension("json.tmp");
        fs::write(&temp_path, data).map_err(|err| io_failure("write", &temp_path, err))?;
        fs::rename(&temp_path, &path).map_err(|err| io_failure("rename", &path, err))?;

        Ok(())
    }
}

/// Converts an I/O error into a storage error.
fn io_failure(action: &str, path: &Path, err: io::Error) -> StorageError {
    StorageError::Failed(format!("could not {action} '{}': {err}", path.display()))
}
