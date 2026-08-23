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

    let scan_result = scanner::scan_directory(root).unwrap();
    let plan = planner::build_plan(root.to_path_buf(), scan_result);

    let directory_result = executor::create_plan_directories(&plan);
    let move_result = executor::execute_plan_moves(&plan);

    assert!(directory_result.failures.is_empty());
    assert!(move_result.failures.is_empty());

    assert_eq!(
        fs::read_to_string(root.join("Images").join("photo.png")).unwrap(),
        "image"
    );
    assert_eq!(
        fs::read_to_string(root.join("Documents").join("report (1).pdf")).unwrap(),
        "new report"
    );
    assert_eq!(
        fs::read_to_string(root.join("Archives").join("archive.tar.gz")).unwrap(),
        "archive"
    );
    assert_eq!(
        fs::read_to_string(root.join("Other").join("mystery.abc")).unwrap(),
        "unknown"
    );

    assert_eq!(
        fs::read_to_string(root.join("Documents").join("report.pdf")).unwrap(),
        "existing report"
    );

    assert!(root.join(".hidden-file").exists());
    assert!(root.join("video.mp4.crdownload").exists());

    assert!(!root.join("photo.png").exists());
    assert!(!root.join("report.pdf").exists());
    assert!(!root.join("archive.tar.gz").exists());
    assert!(!root.join("mystery.abc").exists());
}
