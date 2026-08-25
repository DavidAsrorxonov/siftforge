use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct ScannedFile {
    pub name: String,
    pub path: PathBuf,
}

#[derive(Debug)]
pub struct SkippedEntry {
    pub name: String,
    pub reason: SkipReason,
}

#[derive(Debug)]
pub enum SkipReason {
    Directory,
    Hidden,
    SystemMetadata,
    SiftForgeMetadata,
    IncompleteDownload,
    NotAFile,
    ReadError(String),
}

impl SkipReason {
    pub fn as_message(&self) -> String {
        match self {
            SkipReason::Directory => "directory".to_string(),
            SkipReason::Hidden => "hidden".to_string(),
            SkipReason::SystemMetadata => "system metadata".to_string(),
            SkipReason::SiftForgeMetadata => "SiftForge metadata".to_string(),
            SkipReason::IncompleteDownload => "incomplete download".to_string(),
            SkipReason::NotAFile => "not a regular file".to_string(),
            SkipReason::ReadError(error) => format!("read error: {error}"),
        }
    }
}

#[derive(Debug)]
pub struct ScanResult {
    pub files: Vec<ScannedFile>,
    pub skipped: Vec<SkippedEntry>,
}

pub fn scan_directory(directory: &Path) -> io::Result<ScanResult> {
    let mut files = Vec::new();
    let mut skipped = Vec::new();

    for entry_result in fs::read_dir(directory)? {
        let entry = match entry_result {
            Ok(entry) => entry,
            Err(error) => {
                skipped.push(SkippedEntry {
                    name: "<unknown>".to_string(),
                    reason: SkipReason::ReadError(error.to_string()),
                });
                continue;
            }
        };

        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();

        if is_hidden_name(&name) {
            skipped.push(SkippedEntry {
                name,
                reason: SkipReason::Hidden,
            });
            continue;
        }

        if is_system_metadata(&name) {
            skipped.push(SkippedEntry {
                name,
                reason: SkipReason::SystemMetadata,
            });
            continue;
        }

        if is_siftforge_metadata(&name) {
            skipped.push(SkippedEntry {
                name,
                reason: SkipReason::SiftForgeMetadata,
            });
            continue;
        }

        if is_incomplete_download(&name) {
            skipped.push(SkippedEntry {
                name,
                reason: SkipReason::IncompleteDownload,
            });
            continue;
        }

        let file_type = match entry.file_type() {
            Ok(file_type) => file_type,
            Err(error) => {
                skipped.push(SkippedEntry {
                    name,
                    reason: SkipReason::ReadError(error.to_string()),
                });
                continue;
            }
        };

        if file_type.is_dir() {
            skipped.push(SkippedEntry {
                name,
                reason: SkipReason::Directory,
            });
        } else if file_type.is_file() {
            files.push(ScannedFile { name, path });
        } else {
            skipped.push(SkippedEntry {
                name,
                reason: SkipReason::NotAFile,
            });
        }
    }
    Ok(ScanResult { files, skipped })
}

fn is_hidden_name(name: &str) -> bool {
    name.starts_with(".")
}

fn is_system_metadata(name: &str) -> bool {
    matches!(name, ".DS_Store" | "Thumbs.db" | "desktop.ini")
}

fn is_siftforge_metadata(name: &str) -> bool {
    matches!(name, "siftforge.yml" | "siftforge.yaml")
}

fn is_incomplete_download(name: &str) -> bool {
    let lower_name = name.to_lowercase();

    lower_name.ends_with(".crdownload")
        || lower_name.ends_with(".download")
        || lower_name.ends_with(".part")
        || lower_name.ends_with(".partial")
        || lower_name.ends_with(".tmp")
}

#[cfg(test)]
mod tests {
    use super::{scan_directory, SkipReason};

    #[test]
    fn skips_siftforge_config_files() {
        let temp_dir = tempfile::tempdir().unwrap();

        std::fs::write(temp_dir.path().join("siftforge.yml"), "version: 1").unwrap();
        std::fs::write(temp_dir.path().join("siftforge.yaml"), "version: 1").unwrap();
        std::fs::write(temp_dir.path().join("photo.png"), "image").unwrap();

        let result = scan_directory(temp_dir.path()).unwrap();

        assert_eq!(result.files.len(), 1);
        assert_eq!(result.files[0].name, "photo.png");
        assert_eq!(
            result
                .skipped
                .iter()
                .filter(|entry| matches!(entry.reason, SkipReason::SiftForgeMetadata))
                .count(),
            2
        );
    }
}
