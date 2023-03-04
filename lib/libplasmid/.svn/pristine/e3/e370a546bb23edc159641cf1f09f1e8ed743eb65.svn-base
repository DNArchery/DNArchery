use crate::err::PlasmidError;
use crate::rna::RnaCodon;
use crate::traits::*;

/// Essential Amino Acid
#[derive(Debug, PartialEq, Eq)]
pub enum Eaa {
    /// Any
    Any,
    /// Alanine
    Ala,
    /// Arginine
    Arg,
    /// Asparagine
    Asn,
    /// Aspartic Acid
    Asp,
    /// Aspartate or Asparagine
    Asx,
    /// Cysteine
    Cys,
    /// Glutamine
    Gln,
    /// Glutamic Acid
    Glu,
    /// Glycine
    Gly,
    /// Histidine
    His,
    /// Isoleucine
    Ile,
    /// Leucine
    Leu,
    /// Lysine
    Lys,
    /// Methionine
    Met,
    /// Phenylalanine
    Phe,
    /// Proline
    Pro,
    /// Serine
    Ser,
    /// Stop Codon
    Ter,
    /// Threonine
    Thr,
    /// Tryptophan
    Trp,
    /// Tyrosine
    Tyr,
    /// Valine
    Val,
    /// Gap of indeterminate length
    Gap,
}

impl Eaa {
    pub(crate) fn all_as_str() -> &'static str {
        "-*ABCDEFGHIKLMNPQRSVWXY"
    }

    pub fn is_eaa(c: &char) -> bool {
        Self::all_as_str().contains(*c)
    }
}

impl TryFromLetter for Eaa {
    fn try_from_letter(letter: char) -> anyhow::Result<Self> {
        use self::Eaa::*;
        match letter.to_ascii_uppercase() {
            '-' => Ok(Gap),
            '*' => Ok(Ter),
            'A' => Ok(Ala),
            'B' => Ok(Asx),
            'C' => Ok(Cys),
            'D' => Ok(Asp),
            'E' => Ok(Glu),
            'F' => Ok(Phe),
            'G' => Ok(Gly),
            'H' => Ok(His),
            'I' => Ok(Ile),
            'K' => Ok(Lys),
            'L' => Ok(Leu),
            'M' => Ok(Met),
            'N' => Ok(Asn),
            'P' => Ok(Pro),
            'Q' => Ok(Gln),
            'R' => Ok(Arg),
            'S' => Ok(Ser),
            'T' => Ok(Thr),
            'V' => Ok(Val),
            'W' => Ok(Trp),
            'X' => Ok(Any),
            'Y' => Ok(Tyr),
            _ => bail!(PlasmidError::InvalidAminoAcid { char: letter }),
        }
    }
}

impl ToLetter for Eaa {
    fn to_letter(&self) -> char {
        use self::Eaa::*;
        match self {
            Ala => 'A',
            Any => 'X',
            Arg => 'R',
            Asn => 'N',
            Asp => 'D',
            Asx => 'B',
            Cys => 'C',
            Gap => '-',
            Gln => 'Q',
            Glu => 'E',
            Gly => 'G',
            His => 'H',
            Ile => 'I',
            Leu => 'L',
            Lys => 'K',
            Met => 'M',
            Phe => 'F',
            Pro => 'P',
            Ser => 'S',
            Ter => '*',
            Thr => 'T',
            Trp => 'W',
            Tyr => 'Y',
            Val => 'V',
        }
    }
}

impl ToString for Eaa {
    fn to_string(&self) -> String {
        use self::Eaa::*;
        match self {
            Ala => "Alanine",
            Arg => "Arginine",
            Asn => "Asparagine",
            Asp => "Aspartic acid",
            Asx => "Aspartate or Asparagine",
            Cys => "Cysteine",
            Gap => "(gap)",
            Gln => "Glutamine",
            Glu => "Glutamic acid",
            Gly => "Glycine",
            His => "Histidine",
            Ile => "Isoleucine",
            Leu => "Leucine",
            Lys => "Lysine",
            Met => "Methionine",
            Phe => "Phenylalanine",
            Pro => "Proline",
            Ser => "Serine",
            Ter => "(stop)",
            Thr => "Threonine",
            Trp => "Tryptophan",
            Tyr => "Tyrosine",
            Val => "Valine",
            Any => "(any)",
        }
        .to_string()
    }
}

impl From<&RnaCodon> for Eaa {
    fn from(rna: &RnaCodon) -> Self {
        use crate::eaa::Eaa::*;
        use crate::rna::RnaNucleotide::*;
        match rna.to_triplet_arr() {
            [U, U, U] | [U, U, C] => Phe,
            [U, U, A] | [U, U, G] | [C, U, U] | [C, U, C] | [C, U, A] | [C, U, G] => Leu,
            [A, U, U] | [A, U, C] | [A, U, A] => Ile,
            [A, U, G] => Met,
            [G, U, U] | [G, U, C] | [G, U, A] | [G, U, G] => Val,
            [U, C, U] | [U, C, C] | [U, C, A] | [U, C, G] | [A, G, U] | [A, G, C] => Ser,
            [C, C, U] | [C, C, C] | [C, C, A] | [C, C, G] => Pro,
            [A, C, U] | [A, C, C] | [A, C, A] | [A, C, G] => Thr,
            [G, C, U] | [G, C, C] | [G, C, A] | [G, C, G] => Ala,
            [U, A, U] | [U, A, C] => Tyr,
            [U, A, A] => Ter, // Stop Ochre
            [U, A, G] => Ter, // Stop Amber
            [C, A, U] | [C, A, C] => His,
            [C, A, A] | [C, A, G] => Gln,
            [A, A, U] | [A, A, C] => Asn,
            [A, A, A] | [A, A, G] => Lys,
            [G, A, U] | [G, A, C] => Asp,
            [G, A, A] | [G, A, G] => Glu,
            [U, G, U] | [U, G, C] => Cys,
            [U, G, A] => Ter, // Stop Opal
            [U, G, G] => Trp,
            [C, G, U] | [C, G, C] | [C, G, A] | [C, G, G] => Arg,
            [A, G, A] | [A, G, G] => Arg,
            [G, G, U] | [G, G, C] | [G, G, A] | [G, G, G] => Gly,
        }
    }
}

impl<T> TryFromStr<'_, T> for Eaa
where
    T: AsRef<str>,
{
    fn try_from_str(s: T) -> anyhow::Result<Self> {
        Ok(Eaa::from(&RnaCodon::try_from_str(s)?))
    }
}
