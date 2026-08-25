# Security Policy

SiftForge is a local-first filesystem tool. It does not require accounts, servers, telemetry, cloud services, or file-content uploads.

Because SiftForge moves user files, security and data-loss reports are taken seriously.

## Supported Versions

SiftForge is in early development and has not reached a stable 1.0 release.

Security fixes will target the latest development version until a formal version-support policy is published.

## Reporting a Vulnerability

Do not publicly disclose a vulnerability before maintainers have had a reasonable chance to investigate and fix it.

Until a dedicated security contact is published, open a private security advisory if the repository host supports it. If private advisories are not available, open an issue with minimal public detail and request a private follow-up path.

Please include:

- SiftForge version or commit.
- Operating system.
- Command used.
- Directory structure or minimal reproduction steps.
- Expected behavior.
- Actual behavior.
- Whether files were moved, overwritten, lost, or exposed.

Avoid sharing sensitive filenames or paths unless they are necessary for the report. Redacted examples are preferred.

## Security Scope

Relevant reports include:

- Silent overwrite or data loss.
- Preview mode modifying the filesystem.
- Unsafe path traversal.
- Moving files outside the intended target directory.
- Following symlinks unexpectedly.
- Undo moving unrelated files.
- History written outside the expected local history directory.
- Panic or crash caused by malformed filenames or directory state.
- Unsafe handling of configuration files once configuration support exists.

Out of scope:

- Requests for telemetry, cloud scanning, or remote classification.
- Social engineering unrelated to the project.
- Vulnerabilities requiring modified source code or a malicious local build environment.
- Issues in dependencies that do not affect SiftForge behavior.

## Design Commitments

SiftForge should preserve these security properties:

- No telemetry.
- No cloud upload.
- No file-content inspection unless a future feature explicitly requires it.
- No shell command construction for file operations.
- No silent deletion.
- No silent overwrite.
- Preview remains the default.
- Filesystem changes require explicit `--apply`.
- History remains local.

## Local History Privacy

Operation history contains local absolute paths and filenames. This data should remain on the user’s machine.

History is stored in platform-appropriate local application/state directories:

- macOS: `~/Library/Application Support/siftforge/history/`
- Linux: `$XDG_STATE_HOME/siftforge/history/` or `~/.local/state/siftforge/history/`
- Windows: `%LOCALAPPDATA%\siftforge\history\`

Do not include real history records in public bug reports if they contain sensitive paths.

