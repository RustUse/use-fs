#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

const COMPOUND_EXTENSIONS: [&str; 7] = [
    "tar.gz",
    "tar.bz2",
    "tar.xz",
    "d.ts",
    "module.css",
    "test.ts",
    "spec.ts",
];

/// A string-backed file extension.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FileExtension {
    pub value: String,
}

/// Extracts the last simple extension from a file name or path-like input.
#[must_use]
pub fn extension(input: &str) -> Option<String> {
    let file_name = file_name_segment(input)?;
    let (_, extension) = split_simple_extension(file_name)?;
    Some(extension.to_string())
}

/// Extracts the last simple extension and lowercases it.
#[must_use]
pub fn extension_lowercase(input: &str) -> Option<String> {
    extension(input).map(|value| value.to_ascii_lowercase())
}

/// Returns `true` when a path-like input has a simple extension.
#[must_use]
pub fn has_extension(input: &str) -> bool {
    extension(input).is_some()
}

/// Returns `true` when the last simple extension matches the given candidate.
#[must_use]
pub fn has_extension_eq(input: &str, extension: &str) -> bool {
    let normalized = normalize_extension(extension);
    !normalized.is_empty() && extension_lowercase(input).as_deref() == Some(normalized.as_str())
}

/// Replaces the last simple extension or appends one when missing.
#[must_use]
pub fn with_extension(input: &str, extension: &str) -> String {
    let normalized_input = normalize_path_like(input);
    let normalized_extension = normalize_extension(extension);

    if normalized_input.is_empty() {
        return String::new();
    }

    if normalized_extension.is_empty() {
        return without_extension(&normalized_input);
    }

    let without = without_extension(&normalized_input);
    if without.is_empty() {
        String::new()
    } else {
        format!("{without}.{normalized_extension}")
    }
}

/// Removes the last simple extension while preserving directory segments.
#[must_use]
pub fn without_extension(input: &str) -> String {
    let normalized = normalize_path_like(input);
    let (prefix, file_name) = split_directory_and_file_name(&normalized);
    let Some(file_name) = file_name else {
        return normalized;
    };
    let Some((stem, _)) = split_simple_extension(file_name) else {
        return normalized;
    };

    format!("{prefix}{stem}")
}

/// Normalizes an extension by removing leading dots and lowercasing it.
#[must_use]
pub fn normalize_extension(input: &str) -> String {
    input.trim().trim_start_matches('.').to_ascii_lowercase()
}

/// Returns `true` when the input ends with a supported compound extension.
#[must_use]
pub fn is_compound_extension(input: &str) -> bool {
    compound_extension(input).is_some()
}

/// Extracts a supported compound extension from a file name or path-like input.
#[must_use]
pub fn compound_extension(input: &str) -> Option<String> {
    let file_name = file_name_segment(input)?;
    let normalized = file_name.to_ascii_lowercase();

    for candidate in COMPOUND_EXTENSIONS {
        let suffix = format!(".{candidate}");
        if normalized.ends_with(&suffix) && normalized.len() > suffix.len() {
            return Some(candidate.to_string());
        }
    }

    None
}

fn normalize_path_like(input: &str) -> String {
    input.replace('\\', "/")
}

fn file_name_segment(input: &str) -> Option<&str> {
    let candidate = input.rsplit(['/', '\\']).next().unwrap_or(input);
    (!candidate.is_empty()).then_some(candidate)
}

fn split_directory_and_file_name(input: &str) -> (&str, Option<&str>) {
    match input.rfind('/') {
        Some(index) => {
            let file_name = (index + 1 < input.len()).then(|| &input[index + 1..]);
            (&input[..=index], file_name)
        },
        None => ("", (!input.is_empty()).then_some(input)),
    }
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
