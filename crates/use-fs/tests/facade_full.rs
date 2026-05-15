use use_fs::{
    compound_extension, path, path_kind, sanitize_file_name, slug_file_stem_basic, PathKind,
};

#[test]
fn facade_reexports_are_available() {
    assert_eq!(path_kind("docs/index.md"), PathKind::Relative);
    assert_eq!(path::path_kind("docs/index.md"), PathKind::Relative);
    assert_eq!(
        compound_extension("archive.tar.gz").as_deref(),
        Some("tar.gz")
    );
    assert_eq!(sanitize_file_name("report?.txt"), "report-.txt");
    assert_eq!(
        slug_file_stem_basic("My Report.tar.gz").as_deref(),
        Some("my-report")
    );
}
