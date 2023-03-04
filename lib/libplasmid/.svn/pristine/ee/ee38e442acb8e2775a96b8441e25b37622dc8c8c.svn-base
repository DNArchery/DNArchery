use std::fmt::Display;

use crate::{
    err::{PlasmidError, PlasmidNucleotideType},
    traits::*,
    uni::IupacNucleotide,
};

/// RNA Nucleobase
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DnaNucleotide {
    /// Adenine
    A,
    /// Cytosine
    C,
    /// Guanine
    G,
    /// Thymine
    T,
}

impl DnaNucleotide {
    pub(crate) fn all_as_str() -> &'static str {
        "ACGT"
    }

    pub(crate) fn nucleotide_type() -> PlasmidNucleotideType {
        PlasmidNucleotideType::DNA
    }
}

impl Nucleotide for DnaNucleotide {
    fn complement(&self) -> Self {
        use self::DnaNucleotide::*;
        match self {
            A => T,
            T => A,
            C => G,
            G => C,
        }
    }
}

impl Display for DnaNucleotide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use self::DnaNucleotide::*;
        match self {
            A => write!(f, "Adenine"),
            C => write!(f, "Cytosine"),
            G => write!(f, "Guanine"),
            T => write!(f, "Thymine"),
        }
    }
}

impl TryFromLetter for DnaNucleotide {
    fn try_from_letter(c: char) -> anyhow::Result<Self> {
        use self::DnaNucleotide::*;
        match c.to_ascii_uppercase() {
            'A' => Ok(A),
            'C' => Ok(C),
            'G' => Ok(G),
            'T' => Ok(T),
            _ => bail!(PlasmidError::InvalidNucleotide {
                nucleotide_type: Self::nucleotide_type(),
                char: c
            }),
        }
    }
}

impl ToLetter for DnaNucleotide {
    fn to_letter(&self) -> char {
        use self::DnaNucleotide::*;
        match self {
            A => 'A',
            C => 'C',
            G => 'G',
            T => 'T',
        }
    }
}

impl ToIupac for DnaNucleotide {
    fn to_iupac(&self) -> IupacNucleotide {
        use self::DnaNucleotide::*;
        match self {
            A => IupacNucleotide::A,
            C => IupacNucleotide::C,
            G => IupacNucleotide::G,
            T => IupacNucleotide::T,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DnaNucleotide;

    #[test]
    fn test_dna_base_to_string() {
        assert_eq!(DnaNucleotide::T.to_string(), format!("Thymine"))
    }
}
