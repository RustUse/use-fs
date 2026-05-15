# use-file-name

Practical file-name utility primitives for `RustUse`.

## Experimental

`use-file-name` is experimental while the `use-fs` workspace remains below `0.3.0`.

## Example

```rust
use use_file_name::{is_hidden_file_name, sanitize_file_name};

assert!(is_hidden_file_name(".gitignore"));
assert_eq!(sanitize_file_name("report?.txt"), "report-.txt");
```

## Scope

- file-name extraction from path-like input
- reserved-name detection
- unsafe-character handling
- lightweight sanitization and normalization

## Non-goals

- locale-aware transliteration
- filesystem I/O
- universal security guarantees across all filesystems

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license
