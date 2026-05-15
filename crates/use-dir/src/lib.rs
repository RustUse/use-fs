#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// A string-backed directory path.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DirectoryPath {
    pub value: String,
}

/// Extracts the most relevant directory segment from a path-like input.
#[must_use]
pub fn dir_name(input: &str) -> Option<String> {
    let normalized = normalize_path_like(input);
    let trimmed = trim_trailing_dir_separator(&normalized);
    let (_, segments) = split_root_and_segments(&trimmed);
    let last = segments.last()?.to_string();

    if normalized.ends_with('/') || !looks_like_file_name(last.as_str()) {
        return Some(last);
    }

    (segments.len() >= 2).then(|| segments[segments.len() - 2].to_string())
}

/// Returns `true` when the input is a recognized root directory.
#[must_use]
pub fn is_root_dir(input: &str) -> bool {
    let normalized = normalize_path_like(input);
    let trimmed = trim_trailing_dir_separator(&normalized);
    let (root, segments) = split_root_and_segments(&trimmed);
    root.is_some() && segments.is_empty()
}

/// Returns `true` when the input refers to the current directory.
#[must_use]
pub fn is_current_dir(input: &str) -> bool {
    trim_trailing_dir_separator(&normalize_path_like(input)) == "."
}

/// Returns `true` when the input refers to the parent directory.
#[must_use]
pub fn is_parent_dir(input: &str) -> bool {
    trim_trailing_dir_separator(&normalize_path_like(input)) == ".."
}

/// Normalizes directory separators to `/` and removes trailing separators unless the input is a root.
#[must_use]
pub fn normalize_dir_path(input: &str) -> String {
    trim_trailing_dir_separator(&normalize_path_like(input))
}

/// Ensures a trailing `/` for non-empty directory paths.
#[must_use]
pub fn ensure_dir_trailing_separator(input: &str) -> String {
    let normalized = normalize_path_like(input);
    if normalized.is_empty() || normalized.ends_with('/') {
        return normalized;
    }

    format!("{normalized}/")
}

/// Removes trailing separators while preserving roots.
#[must_use]
pub fn strip_dir_trailing_separator(input: &str) -> String {
    trim_trailing_dir_separator(&normalize_path_like(input))
}

/// Returns the lexical depth of a directory path.
#[must_use]
pub fn path_depth(input: &str) -> usize {
    let normalized = trim_trailing_dir_separator(&normalize_path_like(input));
    let (_, segments) = split_root_and_segments(&normalized);
    segments.len()
}

/// Returns `true` when the input starts with the given directory on segment boundaries.
#[must_use]
pub fn starts_with_dir(input: &str, dir: &str) -> bool {
    let input_normalized = normalize_dir_path(input);
    let dir_normalized = normalize_dir_path(dir);
    if dir_normalized.is_empty() {
        return false;
    }

    let (input_root, input_segments) = split_root_and_segments(&input_normalized);
    let (dir_root, dir_segments) = split_root_and_segments(&dir_normalized);
    if input_root != dir_root || dir_segments.len() > input_segments.len() {
        return false;
    }

    input_segments.starts_with(&dir_segments)
}

/// Computes a lexical relative path when `base` is a prefix of `path`.
#[must_use]
pub fn relative_to_dir(path: &str, base: &str) -> Option<String> {
    let normalized_path = normalize_dir_path(path);
    let normalized_base = normalize_dir_path(base);
    if normalized_base.is_empty() {
        return None;
    }

    let (path_root, path_segments) = split_root_and_segments(&normalized_path);
    let (base_root, base_segments) = split_root_and_segments(&normalized_base);
    if path_root != base_root || base_segments.len() > path_segments.len() {
        return None;
    }

    if !path_segments.starts_with(&base_segments) {
        return None;
    }

    let remainder = &path_segments[base_segments.len()..];
    if remainder.is_empty() {
        Some(String::from("."))
    } else {
        Some(remainder.join("/"))
    }
}

fn normalize_path_like(input: &str) -> String {
    input.replace('\\', "/")
}

fn trim_trailing_dir_separator(input: &str) -> String {
    let mut value = input.to_string();
    while value.ends_with('/') && !is_root_like(&value) {
        value.pop();
    }
    value
}

fn is_root_like(input: &str) -> bool {
    if matches!(input, "/" | "//") {
        return true;
    }

    if drive_root_prefix(input).is_some() && input.len() == 3 {
        return true;
    }

    if let Some(remainder) = input.strip_prefix("//") {
        let segments: Vec<_> = remainder
            .split('/')
            .filter(|segment| !segment.is_empty())
            .collect();
        return segments.len() == 2;
    }

    false
}

fn drive_root_prefix(input: &str) -> Option<&str> {
    let bytes = input.as_bytes();
    if bytes.len() >= 3 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':' && bytes[2] == b'/' {
        Some(&input[..3])
    } else {
        None
    }
}

fn split_root_and_segments(input: &str) -> (Option<String>, Vec<String>) {
    if let Some(remainder) = input.strip_prefix("//") {
        let segments: Vec<_> = remainder
            .split('/')
            .filter(|segment| !segment.is_empty())
            .collect();
        if segments.len() >= 2 {
            let root = format!("//{}/{}", segments[0], segments[1]);
            let rest = segments[2..]
                .iter()
                .map(|segment| (*segment).to_string())
                .collect();
            return (Some(root), rest);
        }
    }

    if let Some(root) = drive_root_prefix(input) {
        let rest = input[root.len()..]
            .split('/')
            .filter(|segment| !segment.is_empty())
            .map(ToOwned::to_owned)
            .collect();
        return (Some(root.to_string()), rest);
    }

    if let Some(remainder) = input.strip_prefix('/') {
        let rest = remainder
            .split('/')
            .filter(|segment| !segment.is_empty())
            .map(ToOwned::to_owned)
            .collect();
        return (Some(String::from("/")), rest);
    }

    let segments = input
        .split('/')
        .filter(|segment| !segment.is_empty())
        .map(ToOwned::to_owned)
        .collect();
    (None, segments)
}

fn looks_like_file_name(segment: &str) -> bool {
    if matches!(segment, "." | "..") || segment.is_empty() {
        return false;
    }

    if segment.starts_with('.') {
        return true;
    }

    match segment.rfind('.') {
        Some(index) if index > 0 && index + 1 < segment.len() => true,
        _ => false,
    }
}
