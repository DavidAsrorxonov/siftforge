# SiftForge Safety Model

SiftForge is designed to organize files without surprising the user or risking silent data loss.

The core safety rule is simple:

```text
Preview first. Apply only when explicitly requested.
```

## Preview Is The Default

Running SiftForge without `--apply` must not modify the filesystem.

Preview mode may:

- read the target directory
- classify files
- build an organization plan
- print proposed moves
- print skipped entries
- print conflict rename counts

Preview mode must not:

- create category directories
- move files
- write operation history
- delete files
- modify configuration

## Apply Is Explicit

Filesystem changes require:

```bash
siftforge <directory> --apply
```

Apply mode:

- creates required category directories
- moves planned files
- refuses to overwrite destinations
- records successful moves and failures
- writes local operation history

## No Overwrites

SiftForge must never overwrite an existing destination file.

If a destination exists, the incoming file is renamed safely:

```text
report.pdf
report (1).pdf
report (2).pdf
```

Compound archive extensions are preserved:

```text
backup.tar.gz
backup (1).tar.gz
```

## Skipped Entries

SiftForge skips the following by default:

- existing directories
- hidden Unix-style names such as `.env`
- system metadata such as `.DS_Store`, `Thumbs.db`, and `desktop.ini`
- incomplete downloads ending with `.crdownload`, `.download`, `.part`, `.partial`, or `.tmp`
- SiftForge config files such as `siftforge.yml` and `siftforge.yaml`
- non-regular filesystem entries

## Undo

Undo restores the latest operation record that has not already been marked undone.

For each move, undo checks:

1. the organized file still exists
2. the original path is available

If either check fails, the file is skipped and reported. Undo continues with the rest of the operation.

After restoring files, undo removes only directories recorded as created by SiftForge, and only if they are empty.

## History Privacy

History records contain local absolute paths and filenames.

History remains local and is stored in platform-specific application/state directories:

- macOS: `~/Library/Application Support/siftforge/history/`
- Linux: `$XDG_STATE_HOME/siftforge/history/` or `~/.local/state/siftforge/history/`
- Windows: `%LOCALAPPDATA%\siftforge\history\`

Do not publish real history records if they contain sensitive paths.

## Non-Goals

SiftForge does not:

- delete user files during organization
- remove duplicates
- upload file data
- require accounts or cloud services
- inspect file contents
- run organized files
- follow symlinks by default
- recurse into subdirectories by default

