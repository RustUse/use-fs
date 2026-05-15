use use_dir::{
    dir_name, ensure_dir_trailing_separator, is_current_dir, is_parent_dir, is_root_dir,
    normalize_dir_path, path_depth, relative_to_dir, starts_with_dir, strip_dir_trailing_separator,
};

#[test]
fn extracts_directory_names() {
    assert_eq!(dir_name("docs/guides/intro.md").as_deref(), Some("guides"));
    assert_eq!(dir_name("docs/guides/").as_deref(), Some("guides"));
    assert_eq!(dir_name("docs").as_deref(), Some("docs"));
    assert_eq!(dir_name("/"), None);
}

#[test]
fn detects_root_current_and_parent_dirs() {
    assert!(is_root_dir("/"));
    assert!(is_root_dir(r"C:\"));
    assert!(is_root_dir(r"\\server\share"));
    assert!(is_current_dir("."));
    assert!(is_current_dir("./"));
    assert!(is_parent_dir(".."));
    assert!(is_parent_dir("../"));
}

#[test]
fn normalizes_and_adjusts_trailing_separators() {
    assert_eq!(normalize_dir_path(r"docs\guides\"), "docs/guides");
    assert_eq!(normalize_dir_path("/"), "/");
    assert_eq!(ensure_dir_trailing_separator("docs"), "docs/");
    assert_eq!(
        ensure_dir_trailing_separator(r"docs\guides"),
        "docs/guides/"
    );
    assert_eq!(strip_dir_trailing_separator("docs/guides/"), "docs/guides");
}

#[test]
fn calculates_path_depth() {
    assert_eq!(path_depth("docs/guides"), 2);
    assert_eq!(path_depth("docs/guides/getting-started"), 3);
    assert_eq!(path_depth("/"), 0);
    assert_eq!(path_depth(r"C:\Users\Joshua"), 2);
    assert_eq!(path_depth(r"\\server\share\folder"), 1);
}

#[test]
fn checks_directory_prefixes() {
    assert!(starts_with_dir("docs/guides/intro.md", "docs"));
    assert!(starts_with_dir("/tmp/logs/app.log", "/tmp"));
    assert!(!starts_with_dir("docs2/intro.md", "docs"));
    assert!(!starts_with_dir("docs/guides", "/tmp"));
}

#[test]
fn computes_relative_paths() {
    assert_eq!(
        relative_to_dir("docs/guides/intro.md", "docs").as_deref(),
        Some("guides/intro.md")
    );
    assert_eq!(relative_to_dir("docs", "docs").as_deref(), Some("."));
    assert_eq!(
        relative_to_dir("/tmp/logs/app.log", "/tmp").as_deref(),
        Some("logs/app.log")
    );
    assert_eq!(relative_to_dir("docs/guides", "other"), None);
}

#[test]
fn handles_empty_input() {
    assert_eq!(dir_name(""), None);
    assert_eq!(normalize_dir_path(""), "");
    assert_eq!(relative_to_dir("", ""), None);
}
