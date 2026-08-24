use serde::{Deserialize, Serialize};
use std::path::PathBuf;

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
    pub destination_path: PathBuf,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum OperationStatus {
    Completed,
    CompletedWithErrors,
}
