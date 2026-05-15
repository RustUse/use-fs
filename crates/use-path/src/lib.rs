#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// Classifies a path-like input using a small lexical model.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathKind {
    /// The input begins with a recognized absolute prefix.
    Absolute,
    /// The input is non-empty and does not begin with a recognized absolute prefix.
    Relative,
    /// The input is an empty string.
    Empty,
}

/// Describes which separator styles appear in a path-like input.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathSeparator {
    /// Only `/` appears.
    Slash,
    /// Only `\` appears.
    Backslash,
    /// Both `/` and `\` appear.
    Mixed,
    /// No path separator appears.
    None,
}

/// A lexical view of a path-like input.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PathParts {
    /// The parent directory portion when one is present.
    pub directory: Option<String>,
    /// The final file-name segment when one is present.
    pub file_name: Option<String>,
    /// The final simple extension when one is present.
    pub extension: Option<String>,
}

/// Returns the lexical kind for a path-like input.
#[must_use]
pub fn path_kind(input: &str) -> PathKind {
    if input.is_empty() {
        return PathKind::Empty;
    }

    if is_absolute_prefix(input) {
        PathKind::Absolute
    } else {
        PathKind::Relative
    }
}

/// Returns `true` when the input begins with a recognized absolute prefix.
#[must_use]
pub fn is_absolute_path(input: &str) -> bool {
    matches!(path_kind(input), PathKind::Absolute)
}

/// Returns `true` when the input is non-empty and not absolute.
#[must_use]
pub fn is_relative_path(input: &str) -> bool {
    matches!(path_kind(input), PathKind::Relative)
}

/// Returns `true` when the input is empty.
#[must_use]
pub fn is_empty_path(input: &str) -> bool {
    matches!(path_kind(input), PathKind::Empty)
}

/// Detects which path separator style appears in the input.
#[must_use]
pub fn detect_path_separator(input: &str) -> PathSeparator {
    match (input.contains('/'), input.contains('\\')) {
        (true, true) => PathSeparator::Mixed,
        (true, false) => PathSeparator::Slash,
        (false, true) => PathSeparator::Backslash,
        (false, false) => PathSeparator::None,
    }
}

/// Normalizes path separators to `/`.
#[must_use]
pub fn normalize_path_separators(input: &str) -> String {
    input.replace('\\', "/")
}

/// Removes trailing separators while preserving recognized roots.
#[must_use]
pub fn trim_trailing_separator(input: &str) -> String {
    let mut normalized = normalize_path_separators(input);

    while normalized.ends_with('/') && !is_root_like(&normalized) {
        normalized.pop();
    }

    normalized
}

/// Ensures a trailing `/` for non-empty input.
#[must_use]
pub fn ensure_trailing_separator(input: &str) -> String {
    let normalized = normalize_path_separators(input);
    if normalized.is_empty() || normalized.ends_with('/') {
        return normalized;
    }

    format!("{normalized}/")
}

/// Joins path parts with `/` while preserving a leading absolute prefix when present.
#[must_use]
pub fn join_path_parts(parts: &[&str]) -> String {
    let mut prefix = String::new();
    let mut segments = Vec::new();

    for part in parts.iter().copied().filter(|part| !part.is_empty()) {
        let normalized = normalize_path_separators(part);
        if prefix.is_empty() {
            if normalized.starts_with("//") {
                prefix = String::from("//");
                segments.extend(
                    normalized[2..]
                        .split('/')
                        .filter(|segment| !segment.is_empty())
                        .map(ToOwned::to_owned),
                );
                continue;
            }

            if let Some(root) = drive_root_prefix(&normalized) {
                prefix = root.to_string();
                segments.extend(
                    normalized[root.len()..]
                        .split('/')
                        .filter(|segment| !segment.is_empty())
                        .map(ToOwned::to_owned),
                );
                continue;
            }

            if let Some(remainder) = normalized.strip_prefix('/') {
                prefix = String::from("/");
                segments.extend(
                    remainder
                        .split('/')
                        .filter(|segment| !segment.is_empty())
                        .map(ToOwned::to_owned),
                );
                continue;
            }
        }

        segments.extend(
            normalized
                .split('/')
                .filter(|segment| !segment.is_empty())
                .map(ToOwned::to_owned),
        );
    }

    match prefix.as_str() {
        "//" => {
            if segments.is_empty() {
                String::from("//")
            } else {
                format!("//{}", segments.join("/"))
            }
        }
        "/" => {
            if segments.is_empty() {
                String::from("/")
            } else {
                format!("/{}", segments.join("/"))
            }
        }
        _ if !prefix.is_empty() => {
            if segments.is_empty() {
                prefix
            } else {
                format!("{prefix}{}", segments.join("/"))
            }
        }
        _ => segments.join("/"),
    }
}

/// Splits a path-like input into normalized non-empty segments.
#[must_use]
pub fn split_path_parts(input: &str) -> Vec<String> {
    normalize_path_separators(input)
        .split('/')
        .filter(|segment| !segment.is_empty())
        .map(ToOwned::to_owned)
        .collect()
}

/// Extracts the lexical parent path when one is present.
#[must_use]
pub fn parent_path(input: &str) -> Option<String> {
    let normalized = trim_trailing_separator(input);
    if normalized.is_empty() || is_root_like(&normalized) {
        return None;
    }

    let slash_index = normalized.rfind('/')?;
    if slash_index == 0 {
        return Some(String::from("/"));
    }

    if slash_index == 2 && drive_root_prefix(&normalized).is_some() {
        return Some(normalized[..=slash_index].to_string());
    }

    if !is_absolute_prefix(&normalized)
        && (normalized[..slash_index].ends_with(':') || normalized[..slash_index].contains(":/"))
    {
        return None;
    }

    let parent = &normalized[..slash_index];
    if parent.is_empty() {
        None
    } else {
        Some(parent.to_string())
    }
}

/// Extracts the final file-name segment from a path-like input.
#[must_use]
pub fn file_name_from_path(input: &str) -> Option<String> {
    let normalized = normalize_path_separators(input);
    let candidate = normalized.rsplit('/').next().unwrap_or(normalized.as_str());
    if candidate.is_empty() {
        return None;
    }

    let trimmed = trim_trailing_separator(&normalized);
    if is_root_like(&trimmed) {
        return None;
    }

    Some(candidate.to_string())
}

/// Extracts the final simple extension from a path-like input.
#[must_use]
pub fn extension_from_path(input: &str) -> Option<String> {
    let file_name = file_name_from_path(input)?;
    let (_, extension) = split_simple_extension(file_name.as_str())?;
    Some(extension.to_string())
}

/// Returns lexical directory, file-name, and extension parts.
#[must_use]
pub fn path_parts(input: &str) -> PathParts {
    let normalized = normalize_path_separators(input);
    if normalized.is_empty() {
        return PathParts::default();
    }

    if normalized.ends_with('/') && !is_root_like(&trim_trailing_separator(&normalized)) {
        return PathParts {
            directory: Some(trim_trailing_separator(&normalized)),
            file_name: None,
            extension: None,
        };
    }

    PathParts {
        directory: parent_path(input),
        file_name: file_name_from_path(input),
        extension: extension_from_path(input),
    }
}

fn is_absolute_prefix(input: &str) -> bool {
    input.starts_with('/')
        || input.starts_with('\\')
        || input.starts_with("//")
        || drive_root_prefix(&normalize_path_separators(input)).is_some()
}

fn drive_root_prefix(input: &str) -> Option<&str> {
    let bytes = input.as_bytes();
    if bytes.len() >= 3 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':' && bytes[2] == b'/' {
        Some(&input[..3])
    } else {
        None
    }
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

fn split_simple_extension(file_name: &str) -> Option<(&str, &str)> {
    let dot_index = file_name.rfind('.')?;
    if dot_index == file_name.len() - 1 {
        return None;
    }

    if dot_index == 0 {
        let nested_dot = file_name[1..].rfind('.')? + 1;
        if nested_dot == file_name.len() - 1 {
            return None;
        }

        return Some((&file_name[..nested_dot], &file_name[nested_dot + 1..]));
    }

    Some((&file_name[..dot_index], &file_name[dot_index + 1..]))
}
