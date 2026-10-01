use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use thiserror::Error;

use crate::archive;
use crate::builtin::{self, BUILTIN_PACKS};
use crate::i18n::messages;
use crate::pack::{self, KeyMap, LoadedPack, PackError, PackInfo};

const USER_ID_PREFIX: &str = "user-";
const ARCHIVE_EXTENSION: &str = "zip";

#[derive(Debug, Error)]
pub enum LibraryError {
    #[error("{0}")]
    Pack(#[from] PackError),
    #[error("{message}", message = messages().pack_not_found)]
    NotFound,
    #[error("{message}", message = messages().builtin_locked)]
    BuiltinLocked,
    #[error("{message}: {0}", message = messages().file_operation_failed)]
    Io(#[from] std::io::Error),
}

pub struct PackEntry {
    pub id: String,
    pub info: PackInfo,
    pub builtin: bool,
}

pub struct Library {
    root: PathBuf,
}

impl Library {
    pub fn new(root: PathBuf) -> std::io::Result<Self> {
        fs::create_dir_all(&root)?;
        Ok(Self { root })
    }

    pub fn entries(&self) -> Vec<PackEntry> {
        let mut entries: Vec<PackEntry> = BUILTIN_PACKS
            .iter()
            .filter_map(|builtin| {
                let files = builtin.files();
                let manifest = files.get(pack::MANIFEST_NAME)?;
                Some(PackEntry {
                    id: builtin.id.to_string(),
                    info: pack::read_info(manifest).ok()?,
                    builtin: true,
                })
            })
            .collect();
        entries.extend(self.user_entries());
        entries
    }

    pub fn load(&self, id: &str) -> Result<LoadedPack, LibraryError> {
        if let Some(builtin) = builtin::find(id) {
            return Ok(pack::load(&builtin.files())?);
        }
        let files = archive::read_pack_files(&self.archive_path(id)?)?;
        Ok(pack::load(&files)?)
    }

    pub fn key_map(&self, id: &str) -> Result<KeyMap, LibraryError> {
        if let Some(builtin) = builtin::find(id) {
            let files = builtin.files();
            let manifest = files
                .get(pack::MANIFEST_NAME)
                .ok_or_else(|| PackError::MissingFile(pack::MANIFEST_NAME.to_string()))?;
            return Ok(pack::read_key_map(manifest)?);
        }
        let manifest = archive::read_manifest(&self.archive_path(id)?)?;
        Ok(pack::read_key_map(&manifest)?)
    }

    pub fn import(&self, source: &Path) -> Result<PackEntry, LibraryError> {
        let files = archive::read_pack_files(source)?;
        let loaded = pack::load(&files)?;
        let id = new_user_id();
        fs::copy(source, self.archive_path(&id)?)?;
        Ok(PackEntry {
            id,
            info: loaded.info,
            builtin: false,
        })
    }

    pub fn remove(&self, id: &str) -> Result<(), LibraryError> {
        if builtin::find(id).is_some() {
            return Err(LibraryError::BuiltinLocked);
        }
        let path = self.archive_path(id)?;
        if !path.exists() {
            return Err(LibraryError::NotFound);
        }
        Ok(fs::remove_file(path)?)
    }

    fn user_entries(&self) -> Vec<PackEntry> {
        let Ok(dir) = fs::read_dir(&self.root) else {
            return Vec::new();
        };
        let mut entries: Vec<PackEntry> = dir
            .filter_map(Result::ok)
            .filter_map(|file| {
                let path = file.path();
                if path.extension()?.to_str()? != ARCHIVE_EXTENSION {
                    return None;
                }
                let id = path.file_stem()?.to_str()?.to_string();
                if !is_user_id(&id) {
                    return None;
                }
                Some(PackEntry {
                    info: archive::read_pack_info(&path).ok()?,
                    id,
                    builtin: false,
                })
            })
            .collect();
        entries.sort_by(|a, b| a.id.cmp(&b.id));
        entries
    }

    fn archive_path(&self, id: &str) -> Result<PathBuf, LibraryError> {
        if !is_user_id(id) {
            return Err(LibraryError::NotFound);
        }
        Ok(self.root.join(format!("{id}.{ARCHIVE_EXTENSION}")))
    }
}

fn is_user_id(id: &str) -> bool {
    id.strip_prefix(USER_ID_PREFIX).is_some_and(|rest| {
        !rest.is_empty() && rest.chars().all(|c| c.is_ascii_digit() || c == '-')
    })
}

fn new_user_id() -> String {
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|elapsed| elapsed.as_millis())
        .unwrap_or_default();
    format!("{USER_ID_PREFIX}{millis}")
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use zip::write::SimpleFileOptions;
    use zip::ZipWriter;

    use super::*;

    fn write_pack_zip(path: &Path, title: &str) {
        let mut writer = ZipWriter::new(fs::File::create(path).unwrap());
        writer
            .start_file("pack.json", SimpleFileOptions::default())
            .unwrap();
        let manifest = format!(r#"{{"title":"{title}","formatVersion":1,"keys":{{}}}}"#);
        writer.write_all(manifest.as_bytes()).unwrap();
        writer.finish().unwrap();
    }

    #[test]
    fn lists_builtin_packs_first() {
        let dir = tempfile::tempdir().unwrap();
        let library = Library::new(dir.path().join("packs")).unwrap();
        let entries = library.entries();
        assert_eq!(entries.len(), BUILTIN_PACKS.len());
        assert!(entries.iter().all(|entry| entry.builtin));
        assert_eq!(entries[0].id, builtin::default_pack().id);
    }

    #[test]
    fn imports_loads_and_removes_a_user_pack() {
        let dir = tempfile::tempdir().unwrap();
        let library = Library::new(dir.path().join("packs")).unwrap();
        let source = dir.path().join("source.zip");
        write_pack_zip(&source, "マイパック");

        let imported = library.import(&source).unwrap();
        assert!(!imported.builtin);
        assert_eq!(imported.info.title, "マイパック");
        assert!(library.load(&imported.id).is_ok());
        assert!(library
            .entries()
            .iter()
            .any(|entry| entry.id == imported.id && !entry.builtin));

        library.remove(&imported.id).unwrap();
        assert!(matches!(
            library.load(&imported.id),
            Err(LibraryError::Pack(_))
        ));
    }

    #[test]
    fn builtin_packs_cannot_be_removed() {
        let dir = tempfile::tempdir().unwrap();
        let library = Library::new(dir.path().join("packs")).unwrap();
        assert!(matches!(
            library.remove(builtin::default_pack().id),
            Err(LibraryError::BuiltinLocked)
        ));
    }

    #[test]
    fn rejects_ids_that_could_escape_the_library() {
        let dir = tempfile::tempdir().unwrap();
        let library = Library::new(dir.path().join("packs")).unwrap();
        for id in ["../secret", "user-../x", "user-", "other"] {
            assert!(matches!(library.load(id), Err(LibraryError::NotFound)));
            assert!(matches!(library.remove(id), Err(LibraryError::NotFound)));
        }
    }

    #[test]
    fn invalid_archive_is_not_imported() {
        let dir = tempfile::tempdir().unwrap();
        let library = Library::new(dir.path().join("packs")).unwrap();
        let source = dir.path().join("broken.zip");
        fs::write(&source, b"not a zip").unwrap();
        assert!(library.import(&source).is_err());
        assert_eq!(library.entries().len(), BUILTIN_PACKS.len());
    }
}
