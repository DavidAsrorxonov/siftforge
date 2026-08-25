## Summary

Describe the change.

## Type of Change

- [ ] Bug fix
- [ ] Feature
- [ ] Documentation
- [ ] Refactor
- [ ] Tests
- [ ] CI/build

## Safety Checklist

- [ ] Preview mode remains non-mutating.
- [ ] `--apply` does not overwrite existing files.
- [ ] Hidden files remain skipped unless explicitly changed.
- [ ] Incomplete downloads remain skipped unless explicitly changed.
- [ ] SiftForge metadata/config files remain skipped.
- [ ] Undo behavior is preserved or updated with tests.
- [ ] History behavior is preserved or updated with tests.

## Verification

Paste commands run:

```bash
cargo fmt --all -- --check
cargo check --all-targets
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
```

## Notes

