use use_file_name::{
    file_name, has_reserved_file_name, is_hidden_file_name, is_safe_file_name, normalize_file_name,
    remove_unsafe_file_name_chars, replace_unsafe_file_name_chars, sanitize_file_name,
};

#[test]
fn extracts_file_names() {
    assert_eq!(file_name("report.txt").as_deref(), Some("report.txt"));
    assert_eq!(file_name(r"docs\report.txt").as_deref(), Some("report.txt"));
    assert_eq!(file_name("docs/"), None);
}

#[test]
fn detects_hidden_names() {
    assert!(is_hidden_file_name(".gitignore"));
    assert!(is_hidden_file_name(".env.local"));
    assert!(!is_hidden_file_name("."));
    assert!(!is_hidden_file_name("report.txt"));
}

#[test]
fn detects_safe_and_unsafe_names() {
    assert!(is_safe_file_name("report.txt"));
    assert!(is_safe_file_name("Quarterly Report.txt"));
    assert!(!is_safe_file_name(" report.txt"));
    assert!(!is_safe_file_name("report?.txt"));
    assert!(!is_safe_file_name("CON.txt"));
}

#[test]
fn detects_reserved_names() {
    assert!(has_reserved_file_name("CON"));
    assert!(has_reserved_file_name("con.txt"));
    assert!(has_reserved_file_name("Lpt1.log"));
    assert!(!has_reserved_file_name("config.txt"));
}

#[test]
fn sanitizes_and_normalizes_names() {
    assert_eq!(sanitize_file_name("report?.txt"), "report-.txt");
    assert_eq!(sanitize_file_name("CON.txt"), "CON_.txt");
    assert_eq!(sanitize_file_name("  draft?.txt  "), "draft-.txt");
    assert_eq!(normalize_file_name("  report.txt  "), "report.txt");
}

#[test]
fn removes_and_replaces_unsafe_characters() {
    assert_eq!(remove_unsafe_file_name_chars("re<po>rt?.txt"), "report.txt");
    assert_eq!(
        replace_unsafe_file_name_chars("re<po>rt?.txt", '_'),
        "re_po_rt_.txt"
    );
    assert_eq!(replace_unsafe_file_name_chars("badname", '_'), "bad_name");
}

#[test]
fn handles_empty_input() {
    assert_eq!(file_name(""), None);
    assert_eq!(sanitize_file_name(""), "file");
    assert_eq!(remove_unsafe_file_name_chars(""), "");
}
