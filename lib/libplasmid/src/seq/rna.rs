use super::GeneticSequence;
use crate::rna::{RnaCodon, RnaNucleotide};

/// Dynamic RNA sequence
pub type RnaSequence = GeneticSequence<RnaNucleotide, RnaCodon>;
