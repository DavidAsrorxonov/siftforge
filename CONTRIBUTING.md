# Contributing to SiftForge

Thanks for your interest in contributing to SiftForge.

SiftForge is a safe, cross-platform Rust CLI for organizing cluttered directories. Contributions should preserve the project’s core safety guarantees: preview by default, explicit apply, no overwrites, deterministic planning, local history, and safe undo.

## Project Status

SiftForge is in early development. The CLI currently supports preview, apply, history, and undo. Configuration files, custom rules, release packaging, and broader documentation are still in progress.

## Development Setup

Requirements:

- Rust stable
- Cargo

Clone the repository and run:

```bash
cargo check
cargo test
```

Run the CLI locally:

```bash
cargo run -- .
```

Use a disposable fixture directory when testing `--apply`:

```bash
cargo run -- /path/to/test-directory
cargo run -- /path/to/test-directory --apply
```

Do not test filesystem-moving behavior against important personal directories until the plan has been previewed and verified.

## Required Checks

Before submitting changes, run:

```bash
cargo fmt
cargo check
cargo clippy -- -D warnings
cargo test
```

All checks should pass without warnings.

## Contribution Guidelines

- Keep changes focused and easy to review.
- Prefer existing module boundaries and project patterns.
- Add tests for behavior changes.
- Avoid introducing dependencies unless they solve a clear problem.
- Do not add telemetry, network calls, cloud services, or file-content uploads.
- Do not add deletion behavior without explicit design discussion.
- Preserve cross-platform behavior by using `Path` and `PathBuf` instead of manual path strings.
- Treat configuration and filesystem state as untrusted input.

## Safety Expectations

Any change that touches scanning, planning, applying, history, or undo should be reviewed carefully for data-loss risk.

Important invariants:

- Preview mode must not modify the filesystem.
- `--apply` must never overwrite existing files.
- Existing directories are skipped by default.
- Hidden and incomplete-download files are skipped by default.
- Undo must skip safely when files are missing or original paths are occupied.
- History must remain local.

## Testing Notes

Use temporary directories for tests that touch the filesystem.

Good test targets include:

- Preview causes no changes.
- Apply moves expected files.
- Conflicts are renamed safely.
- Hidden files are skipped.
- Incomplete downloads are skipped.
- Existing directories are skipped.
- Undo restores files.
- Undo preserves occupied original paths.
- Undo removes only empty directories created by SiftForge.

## Pull Requests

Pull requests should include:

- A clear summary of the change.
- Tests for new or changed behavior.
- Notes about any safety, platform, or compatibility impact.
- Confirmation that the required checks pass.

## License

By contributing, you agree that your contribution will be licensed under the MIT License.

