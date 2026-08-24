use crate::executor::{ExecutionFailure, ExecutionResult};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationRecord {
    pub version: u32,
    pub id: String,
    pub target_directory: PathBuf,
    pub created_at: String,
    pub completed_at: String,
    pub created_directories: Vec<PathBuf>,
    pub moves: Vec<HistoryMove>,
    pub failures: Vec<HistoryFailure>,
    pub status: OperationStatus,
    pub undo: Option<UndoMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryMove {
    pub source_path: PathBuf,
    pub destination_path: PathBuf,
    pub conflict_resolution: String,
    pub completed_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryFailure {
    pub source_path: PathBuf,
    pub destination_path: Option<PathBuf>,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum OperationStatus {
    Completed,
    CompletedWithErrors,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UndoMetadata {
    pub attempted_at: String,
    pub status: UndoStatus,
    pub restored: usize,
    pub skipped: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum UndoStatus {
    Completed,
    CompletedWithWarnings,
}

pub fn build_operation_record(
    target_directory: PathBuf,
    directory_result: &ExecutionResult,
    move_result: &ExecutionResult,
) -> OperationRecord {
    let created_at = current_timestamp();
    let completed_at = created_at.clone();

    let failures = directory_result
        .failures
        .iter()
        .chain(move_result.failures.iter())
        .map(history_failure_from_execution_failure)
        .collect::<Vec<_>>();

    let moves = move_result
        .moved_files
        .iter()
        .map(|moved_file| HistoryMove {
            source_path: moved_file.source_path.clone(),
            destination_path: moved_file.destination_path.clone(),
            conflict_resolution: format!("{:?}", moved_file.conflict_resolution),
            completed_at: completed_at.clone(),
        })
        .collect::<Vec<_>>();

    let status = if failures.is_empty() {
        OperationStatus::Completed
    } else {
        OperationStatus::CompletedWithErrors
    };

    OperationRecord {
        version: 1,
        id: created_at.clone(),
        target_directory,
        created_at,
        completed_at,
        created_directories: directory_result.created_directories.clone(),
        moves,
        failures,
        status,
        undo: None,
    }
}

fn history_failure_from_execution_failure(failure: &ExecutionFailure) -> HistoryFailure {
    HistoryFailure {
        source_path: failure.source_path.clone(),
        destination_path: failure.destination_path.clone(),
        reason: failure.reason.clone(),
    }
}

fn current_timestamp() -> String {
    let now = std::time::SystemTime::now();
    let duration = now
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();

    format!("operation-{}", duration.as_millis())
}

pub fn write_operation_record_to_dir(
    record: &OperationRecord,
    history_dir: &Path,
) -> io::Result<PathBuf> {
    fs::create_dir_all(history_dir)?;

    let file_path = history_dir.join(format!("{}.json", record.id));
    let json = serde_json::to_string_pretty(record)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;

    fs::write(&file_path, json)?;

    Ok(file_path)
}

pub fn default_history_dir() -> io::Result<PathBuf> {
    let base_dir = if cfg!(target_os = "windows") {
        env::var_os("LOCALAPPDATA")
            .map(PathBuf::from)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "LOCALAPPDATA is not set"))?
    } else if cfg!(target_os = "macos") {
        let home = env::var_os("HOME")
            .map(PathBuf::from)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "HOME is not set"))?;

        home.join("Library").join("Application Support")
    } else {
        match env::var_os("XDG_STATE_HOME") {
            Some(path) => PathBuf::from(path),
            None => {
                let home = env::var_os("HOME")
                    .map(PathBuf::from)
                    .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "HOME is not set"))?;

                home.join(".local").join("state")
            }
        }
    };

    Ok(base_dir.join("siftforge").join("history"))
}

pub fn read_operation_record(path: &Path) -> io::Result<OperationRecord> {
    let json = fs::read_to_string(path)?;
    serde_json::from_str(&json).map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))
}

pub fn read_operation_records_from_dir(history_dir: &Path) -> io::Result<Vec<OperationRecord>> {
    if !history_dir.exists() {
        return Ok(Vec::new());
    }

    let mut records = Vec::new();

    for entry_result in fs::read_dir(history_dir)? {
        let entry = entry_result?;
        let path = entry.path();

        if path.extension().and_then(|extension| extension.to_str()) != Some("json") {
            continue;
        }

        let record = read_operation_record(&path)?;
        records.push(record);
    }

    records.sort_by(|left, right| right.id.cmp(&left.id));

    Ok(records)
}

pub fn latest_operation_record_from_dir(history_dir: &Path) -> io::Result<Option<OperationRecord>> {
    let records = read_operation_records_from_dir(history_dir)?;

    Ok(records.into_iter().next())
}

pub fn mark_operation_undone(
    record: &mut OperationRecord,
    restored: usize,
    skipped: usize,
    completed_with_warnings: bool,
) {
    record.undo = Some(UndoMetadata {
        attempted_at: current_timestamp(),
        status: if completed_with_warnings {
            UndoStatus::CompletedWithWarnings
        } else {
            UndoStatus::Completed
        },
        restored,
        skipped,
    });
}

pub fn latest_operation_record_path_from_dir(history_dir: &Path) -> io::Result<Option<PathBuf>> {
    if !history_dir.exists() {
        return Ok(None);
    }

    let mut paths = Vec::new();

    for entry_result in fs::read_dir(history_dir)? {
        let entry = entry_result?;
        let path = entry.path();

        if path.extension().and_then(|extension| extension.to_str()) == Some("json") {
            paths.push(path);
        }
    }

    paths.sort_by(|left, right| {
        let left_name = left
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("");
        let right_name = right
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("");

        right_name.cmp(left_name)
    });

    Ok(paths.into_iter().next())
}

pub fn write_operation_record_to_path(record: &OperationRecord, path: &Path) -> io::Result<()> {
    let json = serde_json::to_string_pretty(record)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;

    fs::write(path, json)
}

pub fn latest_undoable_operation_record_path_from_dir(
    history_dir: &Path,
) -> io::Result<Option<PathBuf>> {
    if !history_dir.exists() {
        return Ok(None);
    }

    let mut paths = Vec::new();

    for entry_result in fs::read_dir(history_dir)? {
        let entry = entry_result?;
        let path = entry.path();

        if path.extension().and_then(|extension| extension.to_str()) == Some("json") {
            paths.push(path);
        }
    }

    paths.sort_by(|left, right| {
        let left_name = left
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("");
        let right_name = right
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("");

        right_name.cmp(left_name)
    });

    for path in paths {
        let record = read_operation_record(&path)?;

        if record.undo.is_none() {
            return Ok(Some(path));
        }
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::latest_operation_record_from_dir;
    use super::latest_undoable_operation_record_path_from_dir;
    use super::read_operation_record;
    use super::read_operation_records_from_dir;
    use super::write_operation_record_to_dir;
    use super::{build_operation_record, OperationStatus};
    use super::{latest_operation_record_path_from_dir, write_operation_record_to_path};
    use super::{mark_operation_undone, UndoStatus};
    use crate::executor::{ExecutedMove, ExecutionFailure, ExecutionResult};
    use crate::planner::ConflictResolution;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn builds_completed_operation_record() {
        let mut directory_result = ExecutionResult::new();
        directory_result
            .created_directories
            .push(PathBuf::from("Images"));

        let mut move_result = ExecutionResult::new();
        move_result.moved_files.push(ExecutedMove {
            source_path: PathBuf::from("photo.png"),
            destination_path: PathBuf::from("Images/photo.png"),
            conflict_resolution: ConflictResolution::None,
        });

        let record = build_operation_record(PathBuf::from("."), &directory_result, &move_result);

        assert_eq!(record.version, 1);
        assert_eq!(record.target_directory, PathBuf::from("."));
        assert_eq!(record.created_directories, vec![PathBuf::from("Images")]);
        assert_eq!(record.moves.len(), 1);
        assert!(record.failures.is_empty());
        assert_eq!(record.status, OperationStatus::Completed);
    }

    #[test]
    fn builds_completed_with_errors_operation_record() {
        let directory_result = ExecutionResult::new();

        let mut move_result = ExecutionResult::new();
        move_result.failures.push(ExecutionFailure {
            source_path: PathBuf::from("missing.pdf"),
            destination_path: Some(PathBuf::from("Documents/missing.pdf")),
            reason: "source file does not exist".to_string(),
        });

        let record = build_operation_record(PathBuf::from("."), &directory_result, &move_result);

        assert_eq!(record.moves.len(), 0);
        assert_eq!(record.failures.len(), 1);
        assert_eq!(record.status, OperationStatus::CompletedWithErrors);
    }

    #[test]
    fn writes_operation_record_to_history_directory() {
        let temp_dir = tempfile::tempdir().unwrap();

        let directory_result = ExecutionResult::new();
        let move_result = ExecutionResult::new();

        let record = build_operation_record(PathBuf::from("."), &directory_result, &move_result);
        let written_path = write_operation_record_to_dir(&record, temp_dir.path()).unwrap();

        assert!(written_path.exists());

        let json = fs::read_to_string(written_path).unwrap();
        let loaded: super::OperationRecord = serde_json::from_str(&json).unwrap();

        assert_eq!(loaded.id, record.id);
        assert_eq!(loaded.version, 1);
        assert_eq!(loaded.status, OperationStatus::Completed);
    }

    #[test]
    fn default_history_dir_ends_with_siftforge_history() {
        let history_dir = super::default_history_dir().unwrap();

        assert!(history_dir.ends_with("siftforge/history"));
    }

    #[test]
    fn reads_operation_records_from_history_directory() {
        let temp_dir = tempfile::tempdir().unwrap();

        let directory_result = ExecutionResult::new();
        let move_result = ExecutionResult::new();

        let record = build_operation_record(PathBuf::from("."), &directory_result, &move_result);
        write_operation_record_to_dir(&record, temp_dir.path()).unwrap();

        let records = read_operation_records_from_dir(temp_dir.path()).unwrap();

        assert_eq!(records.len(), 1);
        assert_eq!(records[0].id, record.id);
    }

    #[test]
    fn returns_latest_operation_record_from_history_directory() {
        let temp_dir = tempfile::tempdir().unwrap();

        let directory_result = ExecutionResult::new();
        let move_result = ExecutionResult::new();

        let mut older = build_operation_record(PathBuf::from("."), &directory_result, &move_result);
        older.id = "operation-1000".to_string();

        let mut newer = build_operation_record(PathBuf::from("."), &directory_result, &move_result);
        newer.id = "operation-2000".to_string();

        write_operation_record_to_dir(&older, temp_dir.path()).unwrap();
        write_operation_record_to_dir(&newer, temp_dir.path()).unwrap();

        let latest = latest_operation_record_from_dir(temp_dir.path())
            .unwrap()
            .unwrap();

        assert_eq!(latest.id, "operation-2000");
    }
    #[test]
    fn marks_operation_as_undone() {
        let directory_result = ExecutionResult::new();
        let move_result = ExecutionResult::new();

        let mut record =
            build_operation_record(PathBuf::from("."), &directory_result, &move_result);

        mark_operation_undone(&mut record, 2, 1, true);

        let undo = record.undo.unwrap();

        assert_eq!(undo.restored, 2);
        assert_eq!(undo.skipped, 1);
        assert_eq!(undo.status, UndoStatus::CompletedWithWarnings);
    }
    #[test]
    fn returns_latest_operation_record_path_from_history_directory() {
        let temp_dir = tempfile::tempdir().unwrap();

        let directory_result = ExecutionResult::new();
        let move_result = ExecutionResult::new();

        let mut older = build_operation_record(PathBuf::from("."), &directory_result, &move_result);
        older.id = "operation-1000".to_string();

        let mut newer = build_operation_record(PathBuf::from("."), &directory_result, &move_result);
        newer.id = "operation-2000".to_string();

        let older_path = write_operation_record_to_dir(&older, temp_dir.path()).unwrap();
        let newer_path = write_operation_record_to_dir(&newer, temp_dir.path()).unwrap();

        let latest_path = latest_operation_record_path_from_dir(temp_dir.path())
            .unwrap()
            .unwrap();

        assert_eq!(latest_path, newer_path);
        assert_ne!(latest_path, older_path);
    }

    #[test]
    fn overwrites_operation_record_at_path() {
        let temp_dir = tempfile::tempdir().unwrap();

        let directory_result = ExecutionResult::new();
        let move_result = ExecutionResult::new();

        let mut record =
            build_operation_record(PathBuf::from("."), &directory_result, &move_result);
        let path = write_operation_record_to_dir(&record, temp_dir.path()).unwrap();

        mark_operation_undone(&mut record, 1, 0, false);
        write_operation_record_to_path(&record, &path).unwrap();

        let loaded = read_operation_record(&path).unwrap();

        assert!(loaded.undo.is_some());
        assert_eq!(loaded.undo.unwrap().restored, 1);
    }
    #[test]
    fn skips_already_undone_records_when_finding_latest_undoable_path() {
        let temp_dir = tempfile::tempdir().unwrap();

        let directory_result = ExecutionResult::new();
        let move_result = ExecutionResult::new();

        let mut older = build_operation_record(PathBuf::from("."), &directory_result, &move_result);
        older.id = "operation-1000".to_string();

        let mut newer = build_operation_record(PathBuf::from("."), &directory_result, &move_result);
        newer.id = "operation-2000".to_string();

        mark_operation_undone(&mut newer, 1, 0, false);

        let older_path = write_operation_record_to_dir(&older, temp_dir.path()).unwrap();
        write_operation_record_to_dir(&newer, temp_dir.path()).unwrap();

        let latest_undoable = latest_undoable_operation_record_path_from_dir(temp_dir.path())
            .unwrap()
            .unwrap();

        assert_eq!(latest_undoable, older_path);
    }
    #[test]
    fn returns_none_when_no_undoable_operation_exists() {
        let temp_dir = tempfile::tempdir().unwrap();

        let directory_result = ExecutionResult::new();
        let move_result = ExecutionResult::new();

        let mut record =
            build_operation_record(PathBuf::from("."), &directory_result, &move_result);
        record.id = "operation-1000".to_string();

        mark_operation_undone(&mut record, 1, 0, false);
        write_operation_record_to_dir(&record, temp_dir.path()).unwrap();

        let latest_undoable =
            latest_undoable_operation_record_path_from_dir(temp_dir.path()).unwrap();

        assert!(latest_undoable.is_none());
    }
}
