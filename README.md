# SiftForge

SiftForge is a safe, cross-platform command-line tool for organizing cluttered directories.

It previews planned changes by default, requires `--apply` before moving files, avoids overwrites, renames conflicts safely, records operation history locally, and supports undo.

Tagline:

```text
Forge order from clutter.
```

## Status

Early development.

SiftForge currently has a working MVP flow:

- Preview organization plans
- Apply organization plans
- Classify files by built-in extension rules
- Safely rename destination conflicts
- Save local operation history
- Show operation history
- Undo the latest undoable operation

Configuration files, custom rules, recursive organization, release packaging, and shell completions are not implemented yet.

## Safety Model

SiftForge is designed around conservative filesystem behavior:

- Preview mode is the default.
- Files are moved only when `--apply` is passed.
- Existing files are never overwritten.
- Destination conflicts are renamed safely, such as `report (1).pdf`.
- Existing directories are skipped by default.
- Hidden files are skipped by default.
- Incomplete downloads are skipped by default.
- Symbolic links and other non-regular entries are skipped.
- Operation history is saved locally.
- Undo restores recorded moves where possible.

SiftForge does not delete user files as part of organization.

## Installation

SiftForge is not published yet.

For local development, run it with Cargo:

```bash
cargo run -- .
```

After release, the intended primary install path is:

```bash
cargo install siftforge
```

## Usage

Preview a directory:

```bash
siftforge ~/Downloads
```

Apply the proposed organization:

```bash
siftforge ~/Downloads --apply
```

Show operation history:

```bash
siftforge history
```

Undo the latest undoable operation:

```bash
siftforge undo
```

Show help:

```bash
siftforge --help
```

Show version:

```bash
siftforge --version
```

When running locally from this repository, prefix commands with `cargo run --`:

```bash
cargo run -- ~/Downloads
cargo run -- ~/Downloads --apply
cargo run -- history
cargo run -- undo
```

## Example Preview

```text
Scanning: /Users/example/Downloads

Proposed organization:

  Images          4 files
  Documents       3 files
  Archives        1 file
  Other           2 files

10 files would be moved.
4 directories would be created.
2 entries would be skipped.
0 conflicts would be renamed safely.

Run `siftforge /Users/example/Downloads --apply` to continue.
```

Preview mode does not create directories, history files, or move files.

## Built-in Categories

SiftForge currently organizes files into these broad categories:

- `Images`
- `Videos`
- `Audio`
- `Documents`
- `Archives`
- `Code`
- `Installers`
- `Other`

Unknown extensions go to `Other`.

Spreadsheets, presentations, Markdown files, and common text/document formats currently belong to `Documents`.

## Skipped Entries

The scanner skips:

- Existing directories
- Hidden Unix-style names such as `.env` and `.DS_Store`
- System metadata such as `.DS_Store`, `Thumbs.db`, and `desktop.ini`
- Incomplete downloads ending with `.crdownload`, `.download`, `.part`, `.partial`, or `.tmp`
- Non-regular filesystem entries

Recursive mode is not implemented yet.

## Conflict Handling

SiftForge never overwrites an existing destination.

If a planned destination already exists:

```text
Documents/report.pdf
```

SiftForge plans a safe renamed destination:

```text
Documents/report (1).pdf
```

Compound archive extensions are preserved:

```text
Archives/backup.tar.gz
Archives/backup (1).tar.gz
```

## History

Applied operations are saved as JSON records in a platform-specific local history directory.

On macOS:

```text
~/Library/Application Support/siftforge/history/
```

On Linux:

```text
$XDG_STATE_HOME/siftforge/history/
```

Fallback on Linux:

```text
~/.local/state/siftforge/history/
```

On Windows:

```text
%LOCALAPPDATA%\siftforge\history\
```

History records include:

- operation ID
- target directory
- created directories
- successful moves
- failures
- operation status
- undo metadata, once undo has been attempted

## Undo

Undo restores the latest operation record that has not already been marked undone.

For each recorded move, SiftForge:

1. Checks that the organized file still exists.
2. Checks that the original source path is available.
3. Moves the file back.
4. Skips safely if either check fails.

After restoring files, SiftForge removes only directories recorded as created by SiftForge, and only if they are empty.

Undo metadata is written back to the history record so repeated undo attempts do not blindly target the same operation.

## Development

Requirements:

- Rust stable
- Cargo

This project declares:

```toml
rust-version = "1.80"
edition = "2021"
```

Run the standard checks:

```bash
cargo fmt
cargo check
cargo clippy -- -D warnings
cargo test
```

Run the CLI locally:

```bash
cargo run -- .
```

Run against a disposable test directory:

```bash
cargo run -- /path/to/test-directory
cargo run -- /path/to/test-directory --apply
```

Do not test `--apply` on a real `Downloads` directory until the behavior you want has been verified with a fixture directory.

## Project Structure

Current source modules:

```text
src/
├── classifier/
├── executor/
├── history/
├── planner/
├── scanner/
├── undo/
├── lib.rs
└── main.rs
```

The library modules contain the reusable core behavior. `main.rs` contains the CLI wiring.

## Roadmap

Near-term work:

- Add license file
- Add CI
- Improve README and documentation as features stabilize
- Add configuration support
- Add custom classification rules
- Add stronger integration tests
- Prepare release packaging

Longer-term possibilities:

- Recursive mode
- JSON output
- Shell completions
- Homebrew tap
- Additional package managers

## Non-goals

SiftForge is not intended to be:

- A file deletion tool
- A duplicate remover
- A file converter
- A cloud service
- An AI content classifier
- A GUI file manager
- A background daemon

## License

MIT
