# use-dir

Practical directory path utility primitives for `RustUse`.

## Experimental

`use-dir` is experimental while the `use-fs` workspace remains below `0.3.0`.

## Example

```rust
use use_dir::{ensure_dir_trailing_separator, path_depth};

assert_eq!(ensure_dir_trailing_separator("docs/guides"), "docs/guides/");
assert_eq!(path_depth("docs/guides/getting-started"), 3);
```

## Scope

- lexical directory helpers
- root, current, and parent directory checks
- trailing-separator normalization
- directory-oriented prefix and relative-path helpers

## Non-goals

- directory creation or reading
- directory walking
- symlink resolution
- filesystem permission management

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license
