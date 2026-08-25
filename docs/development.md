# SiftForge Development

This document describes the local development workflow for SiftForge.

## Requirements

- Rust stable
- Cargo

The crate currently declares:

```toml
edition = "2021"
rust-version = "1.80"
```

## Common Commands

Run the CLI locally:

```bash
cargo run -- .
```

Preview a fixture directory:

```bash
cargo run -- /path/to/test-directory
```

Apply to a fixture directory:

```bash
cargo run -- /path/to/test-directory --apply
```

Do not test `--apply` against important personal directories until preview output has been checked carefully.

## Required Checks

Run before committing or opening a pull request:

```bash
cargo fmt --all -- --check
cargo check --all-targets
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
```

## Project Structure

```text
src/
├── classifier/
├── config/
├── executor/
├── history/
├── planner/
├── scanner/
├── undo/
├── lib.rs
└── main.rs
```

Module responsibilities:

- `scanner`: read direct child entries and skip unsafe/protected entries
- `classifier`: classify files into built-in or custom categories
- `planner`: build preview/apply plans and resolve destination conflicts
- `executor`: create directories and move files safely
- `history`: read/write operation records
- `undo`: restore recorded moves safely
- `config`: load and validate YAML rules
- `main.rs`: CLI wiring

## Testing

Unit tests live next to their modules.

Integration tests live in:

```text
tests/
```

Filesystem tests should use temporary directories.

Important behaviors to cover:

- preview does not modify files
- apply moves files into expected categories
- conflicts are renamed safely
- hidden files are skipped
- incomplete downloads are skipped
- SiftForge config files are skipped
- history records are written
- undo restores files
- undo skips missing or occupied paths safely
- custom config rules override built-ins as expected

## CI

GitHub Actions workflow:

```text
.github/workflows/ci.yml
```

The workflow runs on:

- Ubuntu
- macOS
- Windows

It checks formatting, build, clippy, and tests.

## Release Notes

Update `CHANGELOG.md` for user-visible changes.

Keep README and docs aligned with actual implemented behavior.

