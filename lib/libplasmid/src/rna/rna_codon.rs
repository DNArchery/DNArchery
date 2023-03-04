use crate::{eaa::Eaa, traits::*};

use super::RnaNucleotide;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct RnaCodon {
    triplet: [RnaNucleotide; 3],
}

impl RnaCodon {
    pub fn translate(&self) -> Eaa {
        Eaa::from(self)
    }
}

impl_codon_traits!(RnaNucleotide => RnaCodon);

#[cfg(test)]
mod tests {
    use super::RnaCodon;
    use super::RnaNucleotide::*;
    use crate::traits::*;

    #[test]
    fn test_rna_codon_from_string() -> anyhow::Result<()> {
        let codon = RnaCodon::try_from_str("AUG")?;
        assert_eq!(codon, RnaCodon::from_triplet_arr([A, U, G]));
        Ok(())
    }

    #[test]
    fn test_rna_codon_from_string_psi() -> anyhow::Result<()> {
        let codon = RnaCodon::try_from_str("AΨG")?;
        assert_eq!(codon, [A, U, G].into());
        Ok(())
    }

    #[test]
    fn test_rna_codon_to_string() {
        let codon: RnaCodon = [A, U, G].into();
        assert_eq!(codon.to_string(), "AUG");
    }

    #[test]
    fn test_dna_codon_anticodon() {
        let codon: RnaCodon = [A, U, G].into();
        let anticodon = codon.anticodon();
        assert_eq!(anticodon, [U, A, C].into());
    }
}
