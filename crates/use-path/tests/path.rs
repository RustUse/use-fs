use use_path::{
    detect_path_separator, ensure_trailing_separator, extension_from_path, file_name_from_path,
    is_absolute_path, is_empty_path, is_relative_path, join_path_parts, normalize_path_separators,
    parent_path, path_kind, path_parts, split_path_parts, trim_trailing_separator, PathKind,
    PathParts, PathSeparator,
};

#[test]
fn detects_absolute_relative_and_empty_paths() {
    assert_eq!(path_kind("/tmp/report.txt"), PathKind::Absolute);
    assert_eq!(path_kind(r"C:\Users\Joshua"), PathKind::Absolute);
    assert_eq!(path_kind(r"\\server\share\file.txt"), PathKind::Absolute);
    assert_eq!(path_kind("docs"), PathKind::Relative);
    assert_eq!(path_kind(""), PathKind::Empty);
    assert!(is_absolute_path("/tmp/report.txt"));
    assert!(is_relative_path("docs/report.txt"));
    assert!(is_empty_path(""));
}

#[test]
fn detects_and_normalizes_separators() {
    assert_eq!(
        detect_path_separator("docs/report.txt"),
        PathSeparator::Slash
    );
    assert_eq!(
        detect_path_separator(r"docs\report.txt"),
        PathSeparator::Backslash
    );
    assert_eq!(
        detect_path_separator(r"docs\report.txt/more"),
        PathSeparator::Mixed
    );
    assert_eq!(detect_path_separator("report"), PathSeparator::None);
    assert_eq!(
        normalize_path_separators(r"docs\guides\intro.md"),
        "docs/guides/intro.md"
    );
    assert_eq!(trim_trailing_separator("docs/guides/"), "docs/guides");
    assert_eq!(trim_trailing_separator("/"), "/");
    assert_eq!(ensure_trailing_separator("docs/guides"), "docs/guides/");
}

#[test]
fn joins_and_splits_path_parts() {
    assert_eq!(
        join_path_parts(&["docs", "guides", "intro.md"]),
        "docs/guides/intro.md"
    );
    assert_eq!(
        join_path_parts(&["/tmp", "logs", "app.log"]),
        "/tmp/logs/app.log"
    );
    assert_eq!(join_path_parts(&[r"C:\Users", "Joshua"]), "C:/Users/Joshua");
    assert_eq!(
        split_path_parts(r"C:\Users\Joshua\notes.txt"),
        vec!["C:", "Users", "Joshua", "notes.txt"]
    );
    assert_eq!(split_path_parts("/tmp/logs"), vec!["tmp", "logs"]);
}

#[test]
fn extracts_parent_file_name_and_extension() {
    assert_eq!(
        parent_path("/tmp/logs/app.log").as_deref(),
        Some("/tmp/logs")
    );
    assert_eq!(
        parent_path(r"C:\Users\Joshua\notes.txt").as_deref(),
        Some("C:/Users/Joshua")
    );
    assert_eq!(parent_path("report.txt"), None);
    assert_eq!(
        file_name_from_path("docs/guide.md").as_deref(),
        Some("guide.md")
    );
    assert_eq!(file_name_from_path("docs/"), None);
    assert_eq!(extension_from_path("docs/guide.md").as_deref(), Some("md"));
    assert_eq!(extension_from_path(".gitignore"), None);
    assert_eq!(extension_from_path(".config.json").as_deref(), Some("json"));
}

#[test]
fn extracts_path_parts() {
    assert_eq!(
        path_parts("docs/guide.md"),
        PathParts {
            directory: Some(String::from("docs")),
            file_name: Some(String::from("guide.md")),
            extension: Some(String::from("md")),
        }
    );
    assert_eq!(
        path_parts("docs/guides/"),
        PathParts {
            directory: Some(String::from("docs/guides")),
            file_name: None,
            extension: None,
        }
    );
}

#[test]
fn handles_malformed_and_empty_input() {
    assert_eq!(path_kind("://broken"), PathKind::Relative);
    assert_eq!(parent_path("://broken"), None);
    assert_eq!(file_name_from_path("://broken").as_deref(), Some("broken"));
    assert_eq!(path_parts(""), PathParts::default());
}
