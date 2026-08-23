use std::fs;

use siftforge::executor;
use siftforge::planner;
use siftforge::scanner;

#[test]
fn applies_full_scan_plan_and_move_pipeline() {
    let temp_dir = tempfile::tempdir().unwrap();
    let root = temp_dir.path();

    fs::write(root.join("photo.png"), "image").unwrap();
    fs::write(root.join("report.pdf"), "new report").unwrap();
    fs::write(root.join("archive.tar.gz"), "archive").unwrap();
    fs::write(root.join("mystery.abc"), "unknown").unwrap();
    fs::write(root.join(".hidden-file"), "hidden").unwrap();
    fs::write(root.join("video.mp4.crdownload"), "partial").unwrap();

    fs::create_dir(root.join("Documents")).unwrap();
    fs::write(root.join("Documents").join("report.pdf"), "existing report").unwrap();
}
