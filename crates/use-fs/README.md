# use-fs

Composable filesystem-adjacent primitives for RustUse.

## Experimental

`use-fs` is experimental while the workspace remains below `0.3.0`. Expect API
adjustments as the first wave settles.

## Example

```rust
#[cfg(feature = "full")]
use use_fs::*;

#[cfg(feature = "full")]
{
    assert_eq!(path_kind("docs/guide.md"), PathKind::Relative);
    assert_eq!(normalize_extension(".TAR.GZ"), "tar.gz");
    assert_eq!(sanitize_file_name("report?.txt"), "report-.txt");
}
```

## Scope

- feature-gated access to the focused `use-fs` crates
- direct reexports for path, file-name, extension, directory, and stem helpers
- one dependency for callers that want the full first-wave workspace

## Non-goals

- adding filesystem I/O APIs
- hiding crate boundaries behind heavy abstraction
- replacing `std::path` or `std::fs`

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license
