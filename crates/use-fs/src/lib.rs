#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

#[cfg(feature = "dir")]
pub use use_dir as dir;
#[cfg(feature = "dir")]
pub use use_dir::*;

#[cfg(feature = "extension")]
pub use use_extension as extension;
#[cfg(feature = "extension")]
pub use use_extension::*;

#[cfg(feature = "file-name")]
pub use use_file_name as file_name;
#[cfg(feature = "file-name")]
pub use use_file_name::*;

#[cfg(feature = "file-stem")]
pub use use_file_stem as file_stem;
#[cfg(feature = "file-stem")]
pub use use_file_stem::*;

#[cfg(feature = "path")]
pub use use_path as path;
#[cfg(feature = "path")]
pub use use_path::*;
