# use-file-stem

Practical file stem utility primitives for `RustUse`.

## Experimental

`use-file-stem` is experimental while the `use-fs` workspace remains below `0.3.0`.

## Example

```rust
use use_file_stem::{append_to_file_stem, file_stem_without_compound_extension};

assert_eq!(file_stem_without_compound_extension("archive.tar.gz").as_deref(), Some("archive"));
assert_eq!(append_to_file_stem("report.txt", "-final"), "report-final.txt");
```

## Scope

- file-stem extraction
- compound-extension-aware stem helpers
- stem replacement and formatting
- conservative ASCII slug shaping

## Non-goals

- full slugification systems
- locale-aware casing
- filesystem I/O

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license
