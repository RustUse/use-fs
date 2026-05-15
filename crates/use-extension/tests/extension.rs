use use_extension::{
    compound_extension, extension, extension_lowercase, has_extension, has_extension_eq,
    is_compound_extension, normalize_extension, with_extension, without_extension,
};

#[test]
fn extracts_extensions() {
    assert_eq!(extension("report.TXT").as_deref(), Some("TXT"));
    assert_eq!(extension("archive.tar.gz").as_deref(), Some("gz"));
    assert_eq!(extension(".gitignore"), None);
    assert_eq!(extension(".config.json").as_deref(), Some("json"));
}

#[test]
fn lowercases_and_checks_extensions() {
    assert_eq!(extension_lowercase("report.TXT").as_deref(), Some("txt"));
    assert!(has_extension("report.txt"));
    assert!(!has_extension("README"));
    assert!(has_extension_eq("report.TXT", ".txt"));
    assert!(!has_extension_eq("archive.tar.gz", "tar.gz"));
}

#[test]
fn replaces_and_removes_last_extension() {
    assert_eq!(with_extension("report.txt", "md"), "report.md");
    assert_eq!(with_extension(r"docs\report", ".md"), "docs/report.md");
    assert_eq!(with_extension("archive.tar.gz", "zip"), "archive.tar.zip");
    assert_eq!(without_extension("report.txt"), "report");
    assert_eq!(without_extension(r"docs\report.txt"), "docs/report");
    assert_eq!(without_extension("archive.tar.gz"), "archive.tar");
    assert_eq!(without_extension(".gitignore"), ".gitignore");
}

#[test]
fn normalizes_extensions() {
    assert_eq!(normalize_extension(".TXT"), "txt");
    assert_eq!(normalize_extension("..Tar.Gz"), "tar.gz");
    assert_eq!(normalize_extension(""), "");
}

#[test]
fn detects_compound_extensions() {
    assert_eq!(
        compound_extension("archive.tar.gz").as_deref(),
        Some("tar.gz")
    );
    assert_eq!(
        compound_extension("button.module.css").as_deref(),
        Some("module.css")
    );
    assert_eq!(compound_extension("types.d.ts").as_deref(), Some("d.ts"));
    assert_eq!(compound_extension("spec.ts"), None);
    assert!(is_compound_extension("archive.tar.xz"));
    assert!(!is_compound_extension("report.txt"));
}

#[test]
fn handles_empty_and_malformed_input() {
    assert_eq!(extension(""), None);
    assert_eq!(extension("docs/"), None);
    assert_eq!(without_extension("docs/"), "docs/");
    assert_eq!(with_extension("", "txt"), "");
}
