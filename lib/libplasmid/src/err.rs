use crate::prelude::{DnaNucleotide, IupacNucleotide, RnaNucleotide};

#[derive(Debug)]
pub enum PlasmidNucleotideType {
    DNA,
    RNA,
    IUPAC,
}

impl PlasmidNucleotideType {
    fn allowed_letters(&self) -> &'static str {
        use self::PlasmidNucleotideType::*;
        match self {
            DNA => DnaNucleotide::all_as_str(),
            RNA => RnaNucleotide::all_as_str(),
            IUPAC => IupacNucleotide::all_as_str(),
        }
    }
}

impl std::fmt::Display for PlasmidNucleotideType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use self::PlasmidNucleotideType::*;
        match self {
            DNA => write!(f, "DNA"),
            RNA => write!(f, "RNA"),
            IUPAC => write!(f, "DNA (IUPAC)"),
        }
    }
}

#[derive(Debug)]
pub enum PlasmidError {
    InvalidNucleotide {
        nucleotide_type: PlasmidNucleotideType,
        char: char,
    },
    InvalidNucleotideSequence {
        nucleotide_type: PlasmidNucleotideType,
        seq: String,
    },
    InvalidAminoAcid {
        char: char,
    },
}

impl std::fmt::Display for PlasmidError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use self::PlasmidError::*;
        match self {
            InvalidNucleotide {
                nucleotide_type,
                char,
            } => write!(
                f,
                "Invalid {} nucleotide: {}. Should be one of: {}",
                nucleotide_type,
                char,
                nucleotide_type.allowed_letters()
            ),
            InvalidNucleotideSequence {
                nucleotide_type,
                seq,
            } => write!(
                f,
                "Invalid {} nucleotide sequence: {}. All nucleotides should be one of: {}",
                nucleotide_type,
                seq,
                nucleotide_type.allowed_letters()
            ),
            InvalidAminoAcid { char } => write!(f, "Invalid amino acid: {}", char),
        }
    }
}

impl std::error::Error for PlasmidError {}
