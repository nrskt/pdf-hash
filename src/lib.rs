use std::path::Path;

use pdf::{
    file::{FileOptions, ScanItem},
    object::Resolve,
    PdfError,
};
use sha2::{Digest, Sha256};

type HashString = String;

/// Get the hash value of the specified PDF file
pub fn get_pdf_hash(path: impl AsRef<Path>) -> Result<HashString, PdfError> {
    let bytes = extract_pdf_stream_from_file(path)?;
    Ok(sha256(bytes))
}

fn sha256(bytes: Vec<u8>) -> HashString {
    let mut hasher = Sha256::new();
    hasher.update(bytes.as_slice());
    let result = hasher.finalize();
    format!("{:x}", result)
}

fn extract_pdf_stream_from_file(path: impl AsRef<Path>) -> Result<Vec<u8>, PdfError> {
    let pdf = FileOptions::uncached().open(path)?;
    let resolve = pdf.resolver();

    let mut bytes = Vec::new();
    for scan_item in pdf.scan() {
        let scan_item = scan_item?;
        let stream = extract_pdf_stream_from_scan_item(scan_item, &resolve)?;
        bytes.extend(stream);
    }
    Ok(bytes)
}

/// Get only PdfStream among Objects contained in PDF
fn extract_pdf_stream_from_scan_item(
    item: ScanItem,
    resolve: &impl Resolve,
) -> Result<Vec<u8>, PdfError> {
    match item {
        ScanItem::Object(_r, p) => match p {
            pdf::primitive::Primitive::Stream(stream) => {
                let stream = stream.raw_data(resolve)?;
                let bytes = stream.into_iter().map(|e| *e).collect::<Vec<_>>();
                Ok(bytes)
            }
            // Ignore other primitive types
            _ => Ok(Vec::new()),
        },
        // Ignore everything except Object type
        ScanItem::Trailer(_) => Ok(Vec::new()),
    }
}
