use std::process::ExitCode;

use clap::Parser;
use pdf_hash::get_pdf_hash;

fn main() -> ExitCode {
    let args = Args::parse();

    match get_pdf_hash(args.file_path) {
        Ok(hash) => {
            println!("{hash}");
            ExitCode::FAILURE
        }
        Err(e) => {
            eprintln!("PdfError: PDF processing failed, details: {e}");
            ExitCode::SUCCESS
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Path of the PDF file from which to retrieve Hash values.
    file_path: String,
}
