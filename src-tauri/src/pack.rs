use std::collections::{BTreeMap, HashMap};
use std::io::Cursor;
use std::path::{Component, Path};
use std::time::Duration;

use rand::seq::IndexedRandom;
use rodio::source::Buffered;
use rodio::{Decoder, Source};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::i18n::messages;
use crate::keys::Key;

pub const MANIFEST_NAME: &str = "pack.json";
const SYSTEM_PREFIX: &str = "macos:";
pub const MAX_FILE_BYTES: usize = 5 * 1024 * 1024;
const MAX_DURATION: Duration = Duration::from_secs(10);
const SUPPORTED_FORMAT_VERSION: u32 = 1;
const VERSION_PLACEHOLDER: &str = "{version}";

pub type Sound = Buffered<Decoder<Cursor<Vec<u8>>>>;
pub type PackFiles = HashMap<String, Vec<u8>>;

#[derive(Debug, Error)]
pub enum PackError {
    #[error("{message}: {0}", message = messages().invalid_manifest)]
    InvalidManifest(#[from] serde_json::Error),
    #[error("{message}: {0}", message = messages().unknown_key)]
    UnknownKey(String),
    #[error("{message}: {0}", message = messages().invalid_path)]
    InvalidPath(String),
    #[error("{message}: {0}", message = messages().missing_file)]
    MissingFile(String),
    #[error("{message}: {0}", message = messages().too_large)]
    TooLarge(String),
    #[error("{message}: {0}", message = messages().too_long)]
    TooLong(String),
    #[error("{message}: {0}", message = messages().undecodable)]
    Undecodable(String),
    #[error("{message}: {0}", message = messages().unreadable_file)]
    Io(#[from] std::io::Error),
    #[error("{message}: {0}", message = messages().invalid_zip)]
    Zip(#[from] zip::result::ZipError),
    #[error("{message}", message = messages().too_many_files)]
    TooManyFiles,
    #[error("{message}: {0}", message = messages().symlink)]
    Symlink(String),
    #[error("{message}", message = messages().unsupported_format.replace(VERSION_PLACEHOLDER, &.0.to_string()))]
    UnsupportedFormat(u32),
}

#[derive(Deserialize)]
struct Manifest {
    title: String,
    #[serde(default)]
    description: String,
    keys: BTreeMap<String, String>,
    #[serde(default)]
    fallback: Vec<String>,
    #[serde(rename = "formatVersion")]
    format_version: u32,
}

#[derive(Clone, Serialize)]
pub struct PackInfo {
    pub title: String,
    pub description: String,
}

#[derive(Serialize)]
pub struct KeyMap {
    pub keys: BTreeMap<String, String>,
    pub fallback: Vec<String>,
}

pub struct LoadedPack {
    pub info: PackInfo,
    sounds: Vec<Sound>,
    keys: HashMap<Key, usize>,
    fallback: Vec<usize>,
}

impl LoadedPack {
    pub fn sound(&self, id: usize) -> Option<&Sound> {
        self.sounds.get(id)
    }

    pub fn pick(&self, key: Key) -> Option<usize> {
        if let Some(id) = self.keys.get(&key) {
            return Some(*id);
        }
        match key {
            Key::Char(_) => self.fallback.choose(&mut rand::rng()).copied(),
            _ => None,
        }
    }
}

pub fn read_info(manifest_bytes: &[u8]) -> Result<PackInfo, PackError> {
    let manifest: Manifest = serde_json::from_slice(manifest_bytes)?;
    Ok(PackInfo {
        title: manifest.title,
        description: manifest.description,
    })
}

pub fn read_key_map(manifest_bytes: &[u8]) -> Result<KeyMap, PackError> {
    let manifest: Manifest = serde_json::from_slice(manifest_bytes)?;
    Ok(KeyMap {
        keys: manifest
            .keys
            .into_iter()
            .map(|(name, spec)| (name.to_lowercase(), spec))
            .collect(),
        fallback: manifest.fallback,
    })
}

pub fn load(files: &PackFiles) -> Result<LoadedPack, PackError> {
    let manifest_bytes = files
        .get(MANIFEST_NAME)
        .ok_or_else(|| PackError::MissingFile(MANIFEST_NAME.to_string()))?;
    let manifest: Manifest = serde_json::from_slice(manifest_bytes)?;
    if manifest.format_version > SUPPORTED_FORMAT_VERSION {
        return Err(PackError::UnsupportedFormat(manifest.format_version));
    }

    let mut loader = SoundLoader::new(files);
    let mut keys = HashMap::new();
    for (name, spec) in &manifest.keys {
        let key = name
            .parse::<Key>()
            .map_err(|_| PackError::UnknownKey(name.clone()))?;
        if let Some(id) = loader.load(spec)? {
            keys.insert(key, id);
        }
    }
    let mut fallback = Vec::new();
    for spec in &manifest.fallback {
        if let Some(id) = loader.load(spec)? {
            fallback.push(id);
        }
    }

    Ok(LoadedPack {
        info: PackInfo {
            title: manifest.title,
            description: manifest.description,
        },
        sounds: loader.sounds,
        keys,
        fallback,
    })
}

struct SoundLoader<'a> {
    files: &'a PackFiles,
    sounds: Vec<Sound>,
    loaded: HashMap<String, Option<usize>>,
}

impl<'a> SoundLoader<'a> {
    fn new(files: &'a PackFiles) -> Self {
        Self {
            files,
            sounds: Vec::new(),
            loaded: HashMap::new(),
        }
    }

    fn load(&mut self, spec: &str) -> Result<Option<usize>, PackError> {
        if let Some(id) = self.loaded.get(spec) {
            return Ok(*id);
        }
        let bytes = match spec.strip_prefix(SYSTEM_PREFIX) {
            Some(name) => read_system_sound(name)?,
            None => Some(self.read_pack_file(spec)?),
        };
        let id = match bytes {
            Some(bytes) => {
                self.sounds.push(decode(spec, bytes)?);
                Some(self.sounds.len() - 1)
            }
            None => None,
        };
        self.loaded.insert(spec.to_string(), id);
        Ok(id)
    }

    fn read_pack_file(&self, spec: &str) -> Result<Vec<u8>, PackError> {
        let normalized = normalize_relative_path(spec)?;
        self.files
            .get(&normalized)
            .cloned()
            .ok_or(PackError::MissingFile(normalized))
    }
}

pub fn normalize_relative_path(spec: &str) -> Result<String, PackError> {
    let mut parts = Vec::new();
    for component in Path::new(spec).components() {
        match component {
            Component::Normal(part) => parts.push(part.to_string_lossy().into_owned()),
            _ => return Err(PackError::InvalidPath(spec.to_string())),
        }
    }
    if parts.is_empty() {
        return Err(PackError::InvalidPath(spec.to_string()));
    }
    Ok(parts.join("/"))
}

fn read_system_sound(name: &str) -> Result<Option<Vec<u8>>, PackError> {
    let is_valid = !name.is_empty()
        && name
            .chars()
            .all(|c| c.is_alphanumeric() || matches!(c, '-' | '_' | ' '));
    if !is_valid {
        return Err(PackError::InvalidPath(format!("{SYSTEM_PREFIX}{name}")));
    }
    #[cfg(target_os = "macos")]
    {
        Ok(std::fs::read(format!("/System/Library/Sounds/{name}.aiff")).ok())
    }
    #[cfg(not(target_os = "macos"))]
    {
        Ok(None)
    }
}

fn decode(spec: &str, bytes: Vec<u8>) -> Result<Sound, PackError> {
    if bytes.len() > MAX_FILE_BYTES {
        return Err(PackError::TooLarge(spec.to_string()));
    }
    let decoder =
        Decoder::new(Cursor::new(bytes)).map_err(|_| PackError::Undecodable(spec.to_string()))?;
    let sound = decoder.buffered();
    let max_samples =
        MAX_DURATION.as_secs() * u64::from(sound.sample_rate()) * u64::from(sound.channels());
    let sample_count = sound.clone().take(max_samples as usize + 1).count();
    if sample_count == 0 {
        return Err(PackError::Undecodable(spec.to_string()));
    }
    if sample_count as u64 > max_samples {
        return Err(PackError::TooLong(spec.to_string()));
    }
    Ok(sound)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_paths_outside_the_pack() {
        for spec in ["../a.mp3", "/etc/a.mp3", "./a.mp3", "a/../../b.mp3", ""] {
            assert!(matches!(
                normalize_relative_path(spec),
                Err(PackError::InvalidPath(_))
            ));
        }
        assert_eq!(
            normalize_relative_path("sounds/a.mp3").unwrap(),
            "sounds/a.mp3"
        );
    }

    #[test]
    fn rejects_system_sound_names_with_separators() {
        assert!(read_system_sound("../Pop").is_err());
        assert!(read_system_sound("").is_err());
    }

    #[test]
    fn default_pack_follows_the_reference_keymap() {
        let pack = load(&crate::builtin::default_pack().files()).unwrap();
        assert!(pack.pick(Key::Char('a')).is_some());
        assert!(pack.pick(Key::Char('7')).is_some());
        assert!(pack.pick(Key::Space).is_some());
        assert!(pack.pick(Key::Enter).is_some());
        assert!(pack.pick(Key::Backspace).is_some());
        assert_eq!(pack.pick(Key::Char('a')), pack.pick(Key::Char('o')));
    }

    #[test]
    fn default_pack_sounds_are_audible() {
        let pack = load(&crate::builtin::default_pack().files()).unwrap();
        for key in [Key::Space, Key::Enter, Key::Backspace, Key::Char('a')] {
            let sound = pack.sound(pack.pick(key).unwrap()).unwrap().clone();
            let peak = sound.fold(0f32, |peak, sample| peak.max(sample.abs()));
            assert!(peak > 0.0, "{key:?} is silent");
        }
    }

    #[test]
    fn unassigned_special_keys_stay_silent() {
        let files: PackFiles = [(
            MANIFEST_NAME.to_string(),
            br#"{"title":"t","formatVersion":1,"keys":{},"fallback":[]}"#.to_vec(),
        )]
        .into_iter()
        .collect();
        let pack = load(&files).unwrap();
        assert!(pack.pick(Key::Space).is_none());
        assert!(pack.pick(Key::Char('a')).is_none());
    }

    #[test]
    fn missing_pack_file_is_an_error() {
        let files: PackFiles = [(
            MANIFEST_NAME.to_string(),
            br#"{"title":"t","formatVersion":1,"keys":{"a":"a.mp3"}}"#.to_vec(),
        )]
        .into_iter()
        .collect();
        assert!(matches!(load(&files), Err(PackError::MissingFile(_))));
    }

    #[test]
    fn accepts_supported_format_version() {
        let files: PackFiles = [(
            MANIFEST_NAME.to_string(),
            br#"{"title":"t","formatVersion":1,"keys":{}}"#.to_vec(),
        )]
        .into_iter()
        .collect();
        assert!(load(&files).is_ok());
    }

    #[test]
    fn rejects_missing_format_version() {
        let files: PackFiles = [(
            MANIFEST_NAME.to_string(),
            br#"{"title":"t","keys":{}}"#.to_vec(),
        )]
        .into_iter()
        .collect();
        assert!(matches!(load(&files), Err(PackError::InvalidManifest(_))));
    }

    #[test]
    fn rejects_newer_format_versions() {
        let files: PackFiles = [(
            MANIFEST_NAME.to_string(),
            br#"{"title":"t","formatVersion":2,"keys":{}}"#.to_vec(),
        )]
        .into_iter()
        .collect();
        assert!(matches!(load(&files), Err(PackError::UnsupportedFormat(2))));
    }
}
