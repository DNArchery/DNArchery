use super::GeneticSequence;
use crate::dna::{DnaCodon, DnaNucleotide};

/// Dynamic DNA sequence
pub type DnaSequence = GeneticSequence<DnaNucleotide, DnaCodon>;
