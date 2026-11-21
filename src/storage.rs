//! Persistent storage for application data.

use std::{collections::HashMap, fmt};

use serde::{Serialize, de::DeserializeOwned};

/// Provides a common interface for loading and saving data.
pub trait Storage {
    /// Loads data associated with the given key.
    ///
    /// `Ok(None)` means that no data exists for the given key.
    fn load(&self, key: StorageKey) -> Result<Option<String>, StorageError>;

    /// Saves data associated with the given key.
    ///
    /// Replaces any existing data for the key.
    fn save(&mut self, key: StorageKey, data: &str) -> Result<(), StorageError>;
}

/// Identifies the types of data stored by the application.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StorageKey {
    /// Player progression and statistics.
    PlayerData,

    /// Application preferences and configuration.
    Settings,
}

impl StorageKey {
    /// Returns the persistent name associated with this key.
    ///
    /// Changing a name makes the data unreachable.
    pub fn name(self) -> &'static str {
        match self {
            StorageKey::PlayerData => "player",
            StorageKey::Settings => "settings",
        }
    }
}

/// An error returned when loading or saving data.
///
/// Returned by [`Storage`] implementations and the JSON helpers in this module.
#[derive(Debug, Clone, PartialEq)]
pub enum StorageError {
    /// The storage can't be used at all, so no operation was attempted.
    Unavailable,
    /// The storage was available, but reading or writing failed.
    Failed(String),
    /// Data exists under the key, but it couldn't be parsed.
    Invalid(String),
}

impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StorageError::Unavailable => write!(f, "storage is unavailable"),
            StorageError::Failed(message) => write!(f, "storage operation failed: {message}"),
            StorageError::Invalid(message) => {
                write!(f, "stored data is invalid: {message}")
            }
        }
    }
}

impl std::error::Error for StorageError {}

/// An in-memory storage backend.
///
/// Used in tests, and as a fallback when no persistent storage is available.
#[derive(Debug, Default)]
pub struct MemoryStorage {
    data: HashMap<StorageKey, String>,
}

impl Storage for MemoryStorage {
    fn load(&self, key: StorageKey) -> Result<Option<String>, StorageError> {
        Ok(self.data.get(&key).cloned())
    }

    fn save(&mut self, key: StorageKey, data: &str) -> Result<(), StorageError> {
        self.data.insert(key, data.to_string());
        Ok(())
    }
}

/// Loads and deserializes JSON data associated with the given key.
///
/// Returns the default value if no data exists for the key.
pub fn load_json<T: DeserializeOwned + Default>(
    storage: &dyn Storage,
    key: StorageKey,
) -> Result<T, StorageError> {
    let Some(text) = storage.load(key)? else {
        return Ok(T::default());
    };

    let data = serde_json::from_str(&text).map_err(|err| StorageError::Invalid(err.to_string()))?;
    Ok(data)
}

/// Serializes and saves a value as JSON under the given key.
///
/// Replaces any existing data for the key.
pub fn save_json<T: Serialize>(
    storage: &mut dyn Storage,
    key: StorageKey,
    value: &T,
) -> Result<(), StorageError> {
    let json =
        serde_json::to_string_pretty(value).map_err(|err| StorageError::Failed(err.to_string()))?;

    storage.save(key, &json)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_names_are_stable() {
        assert_eq!(StorageKey::PlayerData.name(), "player");
        assert_eq!(StorageKey::Settings.name(), "settings");
    }
}
