use serde::Serialize;

extern crate seal;

use seal::pair::{
    AlignmentSet, InMemoryAlignmentMatrix, NeedlemanWunsch, Step, SmithWaterman, Strategy
};

use std::fmt::Debug;

#[derive(Serialize)]
pub struct DNAAlignment {
    score: i32,
    alignment_a: String,
    alignment_b: String,
}

fn align_dna<S:Strategy>(dna_a: String, dna_b: String, strategy: S) -> DNAAlignment
where S:Debug,
{
    let sequence_x: Vec<char> = dna_a.chars().collect();
    let sequence_y: Vec<char> = dna_b.chars().collect();

    let alignment_set: Result<AlignmentSet<InMemoryAlignmentMatrix>, _> = AlignmentSet::new(
        sequence_x.len(),
        sequence_y.len(),
        strategy,
        |x, y| sequence_x[x] == sequence_y[y],
    );

    let mut alignment1 = String::new();
    let mut alignment2 = String::new();
    let mut score = 0;

    match alignment_set {
        Ok(alignment_set) => {
            let global_alignment = alignment_set.global_alignment();
            let mut x_vec: Vec<char> = vec![];
            let mut y_vec: Vec<char> = vec![];
            score = global_alignment.score();
            for step in global_alignment.steps() {
                match step {
                    Step::Align { x, y } => {
                        x_vec.push(sequence_x[x]);
                        y_vec.push(sequence_y[y]);
                    }
                    Step::Delete { x } => {
                        x_vec.push(sequence_x[x]);
                        y_vec.push('-');
                    }
                    Step::Insert { y } => {
                        x_vec.push('-');
                        y_vec.push(sequence_y[y]);
                    }
                }
            }
            alignment1 = x_vec.into_iter().collect();
            alignment2 = y_vec.into_iter().collect();
        }
        Err(error) => {
            println!("Failed to generate alignment set due to error:");
            println!("{:?}", error);
        }
    }

    DNAAlignment {
        score: score as i32,
        alignment_a: alignment1,
        alignment_b: alignment2,
    }
}

/// Align two DNA sequences with the Needleman–Wunsch algorithm
/// https://en.wikipedia.org/wiki/Needleman%E2%80%93Wunsch_algorithm
pub fn align_needleman_wunsch(dna_a: String, dna_b: String) -> DNAAlignment {
    let strategy = NeedlemanWunsch::new(1, -1, -1, -1);
    align_dna(dna_a, dna_b, strategy)
}

// Smith-Waterman for DNA sequencing
// https://en.wikipedia.org/wiki/Smith%E2%80%93Waterman_algorithm
pub fn align_smith_waterman(dna_a: String, dna_b: String) -> DNAAlignment {
    let strategy = SmithWaterman::new(2, -1, -1, -1);
    align_dna(dna_a, dna_b, strategy)
}
