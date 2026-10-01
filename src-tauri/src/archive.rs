use std::fs::File;
use std::io::Read;
use std::path::Path;

use zip::ZipArchive;

use crate::pack::{
    normalize_relative_path, read_info, PackError, PackFiles, PackInfo, MANIFEST_NAME,
    MAX_FILE_BYTES,
};

const MAX_ENTRIES: usize = 500;
const MAX_TOTAL_BYTES: usize = 50 * 1024 * 1024;

pub fn read_pack_info(path: &Path) -> Result<PackInfo, PackError> {
    read_info(&read_manifest(path)?)
}

pub fn read_manifest(path: &Path) -> Result<Vec<u8>, PackError> {
    let mut archive = ZipArchive::new(File::open(path)?)?;
    let mut entry = archive
        .by_name(MANIFEST_NAME)
        .map_err(|_| PackError::MissingFile(MANIFEST_NAME.to_string()))?;
    read_limited(&mut entry, MANIFEST_NAME)
}

pub fn read_pack_files(path: &Path) -> Result<PackFiles, PackError> {
    let mut archive = ZipArchive::new(File::open(path)?)?;
    if archive.len() > MAX_ENTRIES {
        return Err(PackError::TooManyFiles);
    }
    let mut files = PackFiles::new();
    let mut total = 0;
    for index in 0..archive.len() {
        let mut entry = archive.by_index(index)?;
        if entry.is_dir() || is_metadata(entry.name()) {
            continue;
        }
        let raw_name = entry.name().to_string();
        if entry.is_symlink() {
            return Err(PackError::Symlink(raw_name));
        }
        let enclosed = entry
            .enclosed_name()
            .ok_or_else(|| PackError::InvalidPath(raw_name.clone()))?;
        let name = normalize_relative_path(&enclosed.to_string_lossy())?;
        let bytes = read_limited(&mut entry, &raw_name)?;
        total += bytes.len();
        if total > MAX_TOTAL_BYTES {
            return Err(PackError::TooLarge(raw_name));
        }
        files.insert(name, bytes);
    }
    Ok(files)
}

fn is_metadata(name: &str) -> bool {
    let file_name = name.rsplit('/').next().unwrap_or(name);
    name.starts_with("__MACOSX/") || file_name == ".DS_Store" || file_name.starts_with("._")
}

fn read_limited(reader: &mut impl Read, name: &str) -> Result<Vec<u8>, PackError> {
    let mut bytes = Vec::new();
    reader
        .take(MAX_FILE_BYTES as u64 + 1)
        .read_to_end(&mut bytes)?;
    if bytes.len() > MAX_FILE_BYTES {
        return Err(PackError::TooLarge(name.to_string()));
    }
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use zip::write::SimpleFileOptions;
    use zip::ZipWriter;

    use super::*;

    fn write_zip(path: &Path, build: impl FnOnce(&mut ZipWriter<File>)) {
        let mut writer = ZipWriter::new(File::create(path).unwrap());
        build(&mut writer);
        writer.finish().unwrap();
    }

    fn add(writer: &mut ZipWriter<File>, name: &str, bytes: &[u8]) {
        writer
            .start_file(name, SimpleFileOptions::default())
            .unwrap();
        writer.write_all(bytes).unwrap();
    }

    const MANIFEST: &[u8] = r#"{"title":"テスト","description":"説明","keys":{}}"#.as_bytes();

    #[test]
    fn reads_manifest_and_files() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("pack.zip");
        write_zip(&path, |writer| {
            add(writer, "pack.json", MANIFEST);
            add(writer, "sounds/a.mp3", b"x");
            add(writer, "__MACOSX/._pack.json", b"x");
            add(writer, ".DS_Store", b"x");
        });
        let info = read_pack_info(&path).unwrap();
        assert_eq!(info.title, "テスト");
        let files = read_pack_files(&path).unwrap();
        assert_eq!(files.len(), 2);
        assert!(files.contains_key("sounds/a.mp3"));
    }

    #[test]
    fn rejects_paths_that_escape_the_pack() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("pack.zip");
        write_zip(&path, |writer| {
            add(writer, "pack.json", MANIFEST);
            add(writer, "../evil.mp3", b"x");
        });
        assert!(matches!(
            read_pack_files(&path),
            Err(PackError::InvalidPath(_))
        ));
    }

    #[test]
    fn rejects_symlinks() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("pack.zip");
        write_zip(&path, |writer| {
            add(writer, "pack.json", MANIFEST);
            writer
                .add_symlink("link.mp3", "/etc/passwd", SimpleFileOptions::default())
                .unwrap();
        });
        assert!(matches!(read_pack_files(&path), Err(PackError::Symlink(_))));
    }

    #[test]
    fn rejects_oversized_files() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("pack.zip");
        write_zip(&path, |writer| {
            add(writer, "pack.json", MANIFEST);
            add(writer, "big.mp3", &vec![0; MAX_FILE_BYTES + 1]);
        });
        assert!(matches!(
            read_pack_files(&path),
            Err(PackError::TooLarge(_))
        ));
    }

    #[test]
    fn missing_manifest_is_reported() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("pack.zip");
        write_zip(&path, |writer| add(writer, "a.mp3", b"x"));
        assert!(matches!(
            read_pack_info(&path),
            Err(PackError::MissingFile(_))
        ));
    }
}
