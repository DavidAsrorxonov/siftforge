# SiftForge Configuration

SiftForge supports YAML configuration for custom organization rules.

Default filename:

```text
siftforge.yml
```

Alternative filename:

```text
siftforge.yaml
```

## Creating A Config

Run:

```bash
siftforge init
```

This creates a starter `siftforge.yml` in the current directory.

SiftForge refuses to overwrite an existing config file.

## Loading Order

Current config lookup order:

1. explicit `--config <path>`
2. `siftforge.yml` in the target directory
3. `siftforge.yaml` in the target directory
4. built-in defaults

Example:

```bash
siftforge ~/Downloads --config ~/rules/siftforge.yml
```

For subcommands such as `rules`, pass root options before the subcommand:

```bash
siftforge --config ~/rules/siftforge.yml rules
```

## Format

```yaml
version: 1

behavior:
  unknown_files: other
  include_hidden: false
  recursive: false
  follow_symlinks: false
  conflict: rename

categories:
  Screenshots:
    filename_starts_with:
      - Screenshot
      - Screen Shot
    extensions:
      - png
      - jpg
      - jpeg

  University:
    filename_contains:
      - assignment
      - lecture
      - syllabus
    extensions:
      - pdf
      - docx
      - pptx
```

## Rule Priority

Current priority:

1. user `filename_starts_with`
2. user `filename_contains`
3. user `extensions`
4. built-in extension mappings
5. `Other`

The first matching user category in config order wins within each rule type.

## Extension Rules

Extensions should be written without a leading dot:

```yaml
categories:
  University:
    extensions:
      - pdf
      - docx
```

Allowed:

```text
pdf
tar.gz
png
```

Rejected:

```text
.pdf
```

## Filename Rules

Starts-with rules:

```yaml
categories:
  Screenshots:
    filename_starts_with:
      - Screenshot
```

Contains rules:

```yaml
categories:
  University:
    filename_contains:
      - assignment
```

Filename matching is currently case-sensitive.

## Category Names

Category names currently represent one child directory under the target directory.

Allowed:

```text
Images
University
Screenshots
```

Rejected:

```text
Media/Images
../Outside
C:\Outside
```

## Validation

SiftForge rejects config files with:

- unsupported config versions
- empty category names
- category path separators
- path traversal markers
- absolute-looking category names
- rules with no match conditions
- empty extensions
- extensions starting with `.`

Invalid config exits before scanning or moving files.

## Viewing Rules

Run:

```bash
siftforge rules
```

With an explicit config:

```bash
siftforge --config ./siftforge.yml rules
```

