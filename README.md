# RustUse use-fs

Small filesystem-adjacent utility primitives for Rust.

`use-fs` is a RustUse set for lightweight path, file-name, extension, directory,
and file-stem helpers that stay lexical and string-oriented.

## Experimental

This workspace is experimental while it remains below `0.3.0`. Expect the
public API to stay small and practical, but still evolve as the RustUse
filesystem-adjacent surface becomes clearer.

## Workspace crates

- `use-fs`: thin facade crate for the full workspace
- `use-path`: practical path utility primitives
- `use-file-name`: file-name validation and sanitization helpers
- `use-extension`: file extension and compound-extension helpers
- `use-dir`: directory path helpers
- `use-file-stem`: file-stem helpers

## Scope

- string-oriented path inspection and formatting
- relative and absolute path classification
- file-name safety checks and sanitization helpers
- extension and compound-extension inspection
- directory-oriented lexical helpers
- file-stem extraction and simple stem formatting

## Non-goals

- file reading or writing
- directory traversal or walking
- canonicalization against the live filesystem
- shell expansion
- glob matching or gitignore parsing
- symlink resolution
- filesystem permission management

## Relationship to sibling crates and std

`use-fs` stays focused on filesystem-adjacent lexical helpers.

- `use-pattern` should own glob and wildcard matching.
- `use-web` should own URLs, URIs, query strings, and route paths.
- `use-media` should own media-format and metadata helpers.
- `std::path` and `std::fs` remain the standard library foundations for real
  path types and filesystem I/O.

## Example

```rust
use use_path::{join_path_parts, normalize_path_separators, path_kind, PathKind};

assert_eq!(path_kind("/tmp/report.txt"), PathKind::Absolute);
assert_eq!(normalize_path_separators(r"assets\icons\logo.svg"), "assets/icons/logo.svg");
assert_eq!(join_path_parts(&["assets", "icons", "logo.svg"]), "assets/icons/logo.svg");
```

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0, in `LICENSE-APACHE`
- MIT license, in `LICENSE-MIT`
