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

/// A string-backed file stem.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FileStem {
    pub value: String,
}

/// Extracts the simple file stem from a file name or path-like input.
#[must_use]
pub fn file_stem(input: &str) -> Option<String> {
    let file_name = file_name_segment(input)?;
    match split_simple_extension(file_name) {
        Some((stem, _)) => Some(stem.to_string()),
        None => Some(file_name.to_string()),
    }
}

/// Extracts the file stem while removing a supported compound extension when present.
#[must_use]
pub fn file_stem_without_compound_extension(input: &str) -> Option<String> {
    let file_name = file_name_segment(input)?;

    if let Some(extension) = compound_extension(file_name) {
        let stem = &file_name[..file_name.len() - extension.len() - 1];
        return (!stem.is_empty()).then(|| stem.to_string());
    }

    file_stem(file_name)
}

/// Returns `true` when the input has a recoverable file stem.
#[must_use]
pub fn has_file_stem(input: &str) -> bool {
    file_stem(input).is_some()
}

/// Replaces the file stem while preserving the original extension suffix.
#[must_use]
pub fn with_file_stem(input: &str, stem: &str) -> String {
    replace_stem(input, |current| {
        let _ = current;
        stem.to_string()
    })
}

/// Appends a suffix to the file stem while preserving the original extension suffix.
#[must_use]
pub fn append_to_file_stem(input: &str, suffix: &str) -> String {
    replace_stem(input, |current| format!("{current}{suffix}"))
}

/// Prepends a prefix to the file stem while preserving the original extension suffix.
#[must_use]
pub fn prepend_to_file_stem(input: &str, prefix: &str) -> String {
    replace_stem(input, |current| format!("{prefix}{current}"))
}

/// Converts a file stem into a conservative ASCII slug.
#[must_use]
pub fn slug_file_stem_basic(input: &str) -> Option<String> {
    let stem = file_stem_without_compound_extension(input).or_else(|| file_stem(input))?;
    let mut slug = String::new();
    let mut previous_was_separator = false;

    for character in stem.trim().chars() {
        let lowered = character.to_ascii_lowercase();

        if lowered.is_ascii_alphanumeric() {
            slug.push(lowered);
            previous_was_separator = false;
        } else if !slug.is_empty() && !previous_was_separator {
            slug.push('-');
            previous_was_separator = true;
        }
    }

    while slug.ends_with('-') {
        slug.pop();
    }

    (!slug.is_empty()).then_some(slug)
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

fn compound_extension(file_name: &str) -> Option<&'static str> {
    let normalized = file_name.to_ascii_lowercase();

    for candidate in COMPOUND_EXTENSIONS {
        let suffix = format!(".{candidate}");
        if normalized.ends_with(&suffix) && normalized.len() > suffix.len() {
            return Some(candidate);
        }
    }

    None
}

fn extension_suffix(file_name: &str) -> String {
    if let Some(extension) = compound_extension(file_name) {
        return format!(".{extension}");
    }

    split_simple_extension(file_name)
        .map(|(_, extension)| format!(".{extension}"))
        .unwrap_or_default()
}

fn operation_stem(file_name: &str) -> String {
    file_stem_without_compound_extension(file_name)
        .or_else(|| file_stem(file_name))
        .unwrap_or_default()
}

fn replace_stem(input: &str, update: impl FnOnce(&str) -> String) -> String {
    let normalized = normalize_path_like(input);
    let (prefix, file_name) = split_directory_and_file_name(&normalized);
    let Some(file_name) = file_name else {
        return normalized;
    };

    let suffix = extension_suffix(file_name);
    let next_stem = update(operation_stem(file_name).as_str());

    format!("{prefix}{next_stem}{suffix}")
}
