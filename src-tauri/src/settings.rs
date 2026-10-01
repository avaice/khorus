use std::fs;
use std::path::PathBuf;
use std::sync::{Mutex, PoisonError};

use serde::{Deserialize, Serialize};

use crate::i18n::Language;
use crate::volume::Volumes;

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Settings {
    pub selected_pack: Option<String>,
    pub volumes: Volumes,
    pub language: Language,
}

pub struct SettingsStore {
    path: PathBuf,
    current: Mutex<Settings>,
}

impl SettingsStore {
    pub fn load(path: PathBuf) -> Self {
        let current = fs::read(&path)
            .ok()
            .and_then(|bytes| serde_json::from_slice(&bytes).ok())
            .unwrap_or_default();
        Self {
            path,
            current: Mutex::new(current),
        }
    }

    pub fn get(&self) -> Settings {
        self.current
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone()
    }

    pub fn update(&self, change: impl FnOnce(&mut Settings)) -> std::io::Result<()> {
        let mut current = self.current.lock().unwrap_or_else(PoisonError::into_inner);
        change(&mut current);
        fs::write(&self.path, serde_json::to_vec_pretty(&*current)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn persists_updates_across_loads() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("settings.json");
        let store = SettingsStore::load(path.clone());
        store
            .update(|settings| {
                settings.selected_pack = Some("click".to_string());
                settings.volumes.space = 0.4;
            })
            .unwrap();

        let reloaded = SettingsStore::load(path).get();
        assert_eq!(reloaded.selected_pack.as_deref(), Some("click"));
        assert_eq!(reloaded.volumes.space, 0.4);
        assert_eq!(reloaded.volumes.enter, 1.0);
    }

    #[test]
    fn broken_file_falls_back_to_defaults() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("settings.json");
        fs::write(&path, b"{").unwrap();
        let settings = SettingsStore::load(path).get();
        assert!(settings.selected_pack.is_none());
        assert_eq!(settings.volumes, Volumes::default());
    }
}
