use use_file_stem::{
    append_to_file_stem, file_stem, file_stem_without_compound_extension, has_file_stem,
    prepend_to_file_stem, slug_file_stem_basic, with_file_stem,
};

#[test]
fn extracts_file_stems() {
    assert_eq!(file_stem("report.txt").as_deref(), Some("report"));
    assert_eq!(file_stem("archive.tar.gz").as_deref(), Some("archive.tar"));
    assert_eq!(file_stem(".gitignore").as_deref(), Some(".gitignore"));
    assert_eq!(file_stem(r"docs\report.txt").as_deref(), Some("report"));
}

#[test]
fn extracts_compound_extension_stems() {
    assert_eq!(
        file_stem_without_compound_extension("archive.tar.gz").as_deref(),
        Some("archive")
    );
    assert_eq!(
        file_stem_without_compound_extension("button.module.css").as_deref(),
        Some("button")
    );
    assert_eq!(
        file_stem_without_compound_extension("types.d.ts").as_deref(),
        Some("types")
    );
    assert_eq!(
        file_stem_without_compound_extension("report.txt").as_deref(),
        Some("report")
    );
}

#[test]
fn detects_stem_presence() {
    assert!(has_file_stem("report.txt"));
    assert!(!has_file_stem(""));
    assert!(!has_file_stem("docs/"));
}

#[test]
fn replaces_and_formats_stems() {
    assert_eq!(with_file_stem("report.txt", "summary"), "summary.txt");
    assert_eq!(with_file_stem("archive.tar.gz", "backup"), "backup.tar.gz");
    assert_eq!(
        with_file_stem(r"docs\report.txt", "summary"),
        "docs/summary.txt"
    );
    assert_eq!(
        append_to_file_stem("report.txt", "-final"),
        "report-final.txt"
    );
    assert_eq!(
        append_to_file_stem("archive.tar.gz", "-v2"),
        "archive-v2.tar.gz"
    );
    assert_eq!(
        prepend_to_file_stem("report.txt", "draft-"),
        "draft-report.txt"
    );
}

#[test]
fn slugs_file_stems() {
    assert_eq!(
        slug_file_stem_basic("My Report.tar.gz").as_deref(),
        Some("my-report")
    );
    assert_eq!(
        slug_file_stem_basic("Budget 2026 Final.txt").as_deref(),
        Some("budget-2026-final")
    );
    assert_eq!(slug_file_stem_basic("***"), None);
}

#[test]
fn handles_empty_input() {
    assert_eq!(file_stem(""), None);
    assert_eq!(file_stem_without_compound_extension(""), None);
    assert_eq!(with_file_stem("", "summary"), "");
}
