use rust_genomics::{Sequence, LORF};

use actix_web::Either; // either type for web response

/// Carve the Longest Open Reading Frame (ORF) in a sequence from codon indexes
fn carve_lorf(sequence: String, lorf: LORF) -> Either<String, Vec<String>> {
    match lorf {
        LORF::One(codon_index) => {
            Either::Left(
                sequence[codon_index[0]..codon_index[1]].to_string()
            )
        },
        LORF::Many(codon_indexes) => {
            let mut lorfs = Vec::new();
            for codon_index in codon_indexes {
                lorfs.push(
                    sequence[codon_index[0]..codon_index[1]].to_string()
                );
            }
            Either::Right(lorfs)
        },
    }
}

/// Nucleotide at a specific index in a sequence
pub fn nucleotide_index(sequence: String, index: usize) -> String {
    let seq = Sequence::new(sequence);
    match seq.find_at_index(index) {
        Ok(n) => format!("{}", n),
        Err(e) => format!("{e}")
    }
}

/// Takes a sequence and returns a vector of codons at 3 reading frames
pub fn codon_frames(sequence: String) -> String {
    let seq = Sequence::new(sequence);
    let frames = seq.return_reading_frames();
    format!("{:?}", frames)
}

/// Takes a sequence and returns the Longest Open Reading Frame (LORF)
pub fn seq_lorf(sequence: String) -> Either<String, Vec<String>> {
    let mut seq = Sequence::new(sequence.to_owned());
    let lorfs = seq.find_lorf();

    // Carve the LORF string from the sequence
    carve_lorf(sequence, lorfs)
}

/// Generate a random genomic sequence of a given length
pub fn seq_random(len: i64) -> String {
    let sequence = Sequence::gen_random_seq(len);
    sequence.seq
}