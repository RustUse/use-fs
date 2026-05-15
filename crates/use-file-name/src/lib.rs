#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

const RESERVED_NAMES: [&str; 22] = [
    "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8",
    "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
];

/// A string-backed file name.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FileName {
    pub value: String,
}

/// Extracts the final file-name segment from a path-like input.
#[must_use]
pub fn file_name(input: &str) -> Option<String> {
    file_name_candidate(input).map(ToOwned::to_owned)
}

/// Returns `true` when the extracted file name is a hidden dotfile.
#[must_use]
pub fn is_hidden_file_name(input: &str) -> bool {
    file_name(input).as_deref().is_some_and(|value| {
        value.starts_with('.') && value.len() > 1 && !matches!(value, "." | "..")
    })
}

/// Returns `true` when the extracted file name avoids reserved names and unsafe characters.
#[must_use]
pub fn is_safe_file_name(input: &str) -> bool {
    let Some(candidate) = file_name_candidate(input) else {
        return false;
    };

    !candidate.is_empty()
        && !matches!(candidate, "." | "..")
        && !candidate.starts_with(' ')
        && !candidate.ends_with([' ', '.'])
        && !has_reserved_file_name(candidate)
        && !candidate.chars().any(is_unsafe_character)
}

/// Sanitizes a file name by trimming, replacing unsafe characters, and disarming reserved names.
#[must_use]
pub fn sanitize_file_name(input: &str) -> String {
    let candidate = normalize_file_name(input);
    let mut value = replace_unsafe_file_name_chars(candidate.as_str(), '-');
    value = value.trim_matches(' ').trim_end_matches('.').to_string();

    if value.is_empty() || matches!(value.as_str(), "." | "..") {
        return String::from("file");
    }

    if has_reserved_file_name(&value) {
        if let Some((base, rest)) = value.split_once('.') {
            value = format!("{base}_.{rest}");
        } else {
            value.push('_');
        }
    }

    value
}

/// Normalizes a file name by trimming surrounding whitespace.
#[must_use]
pub fn normalize_file_name(input: &str) -> String {
    file_name_candidate(input).unwrap_or("").trim().to_string()
}

/// Returns `true` when the extracted file name is a reserved Windows name.
#[must_use]
pub fn has_reserved_file_name(input: &str) -> bool {
    let Some(candidate) = file_name_candidate(input) else {
        return false;
    };

    let trimmed = candidate.trim().trim_end_matches([' ', '.']);
    if trimmed.is_empty() {
        return false;
    }

    let base = trimmed
        .split('.')
        .next()
        .unwrap_or(trimmed)
        .trim_end_matches([' ', '.']);
    RESERVED_NAMES.contains(&base.to_ascii_uppercase().as_str())
}

/// Removes unsafe characters and control characters from a file name.
#[must_use]
pub fn remove_unsafe_file_name_chars(input: &str) -> String {
    file_name_candidate(input)
        .unwrap_or("")
        .chars()
        .filter(|character| !is_unsafe_character(*character))
        .collect()
}

/// Replaces unsafe characters and control characters with a safe replacement.
#[must_use]
pub fn replace_unsafe_file_name_chars(input: &str, replacement: char) -> String {
    let replacement = if is_unsafe_character(replacement) {
        '-'
    } else {
        replacement
    };

    file_name_candidate(input)
        .unwrap_or("")
        .chars()
        .map(|character| match character {
            '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*' => replacement,
            character if character.is_control() => replacement,
            _ => character,
        })
        .collect()
}

fn file_name_candidate(input: &str) -> Option<&str> {
    if input.is_empty() {
        return None;
    }

    let candidate = input.rsplit(['/', '\\']).next().unwrap_or(input);
    (!candidate.is_empty()).then_some(candidate)
}

fn is_unsafe_character(character: char) -> bool {
    matches!(
        character,
        '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*'
    ) || character.is_control()
}
