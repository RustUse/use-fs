# use-extension

Practical file extension utility primitives for `RustUse`.

## Experimental

`use-extension` is experimental while the `use-fs` workspace remains below `0.3.0`.

## Example

```rust
use use_extension::{compound_extension, extension_lowercase};

assert_eq!(extension_lowercase("ARCHIVE.TAR.GZ").as_deref(), Some("gz"));
assert_eq!(compound_extension("archive.tar.gz").as_deref(), Some("tar.gz"));
```

## Scope

- extension extraction and normalization
- compound-extension inspection for common patterns
- simple extension replacement and removal

## Non-goals

- MIME sniffing
- file-format parsing
- filesystem I/O

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license
