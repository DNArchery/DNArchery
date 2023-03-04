use rust_genomics::FASTA;
use std::io::{Result, Error};

use std::path::Path;

/// Tooltips constants for specification schema
const _LOAD_FROM_FASTA: &str = "Load a FASTA file and return the longest open reading frame (LORF) in the file";

/// Find the Longest Open Reading Frame (ORF) in a FASTA file
/// https://en.wikipedia.org/wiki/Open_reading_frame
/// 
/// Returns a tuple with the longest ORF and its length
pub fn lorf_from_fasta(file_path: &str) -> Result<(String, usize)> {
    if !Path::new(file_path).exists() {
        return Err(
            Error::new(
                std::io::ErrorKind::NotFound,
                format!("File {} not found", file_path)
            ))
    }

    // utilizes rayon here for parallel processing
    let mut fasta = FASTA::rayon_read_fasta(file_path);

    let lorfs = fasta.find_lorfs(true);
    
    Ok(
        (format!("{:?}", lorfs), lorfs.len())
    )
}