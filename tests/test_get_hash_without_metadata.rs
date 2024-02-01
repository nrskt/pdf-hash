use std::path::{Path, PathBuf};

use pdf_hash::get_pdf_hash;

#[test]
fn test_get_hash_value_without_metadata() {
    let test_file_path = test_file_path("dummy.pdf");
    let hash_value = get_pdf_hash(test_file_path).unwrap();

    assert_eq!(
        hash_value,
        "86e2b0250074e9a83254c886dc0fca519ff7dd38dd5c3188fc5068911cf6dee9"
    );
}

fn test_file_path(file_name: impl AsRef<Path>) -> impl AsRef<Path> {
    let mut file_path = PathBuf::new();
    file_path.push(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    file_path.push("tests");
    file_path.push("pdf");
    file_path.push(file_name);
    file_path
}
