# use-path

Practical path utility primitives for `RustUse`.

## Experimental

`use-path` is experimental while the `use-fs` workspace remains below `0.3.0`.

## Example

```rust
use use_path::{PathKind, normalize_path_separators, path_kind};

assert_eq!(path_kind("C:/Users/example"), PathKind::Absolute);
assert_eq!(normalize_path_separators(r"src\lib.rs"), "src/lib.rs");
```

## Scope

- lexical path classification
- separator detection and normalization
- simple path joining and splitting
- parent, file-name, and extension extraction

## Non-goals

- filesystem access
- canonicalization against the live filesystem
- shell expansion
- symlink resolution

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license
