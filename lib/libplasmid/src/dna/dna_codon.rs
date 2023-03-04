use std::collections::HashMap;

use crate::{
    eaa::Eaa,
    rna::{RnaCodon, RnaNucleotide},
    traits::*,
};

use super::DnaNucleotide;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct DnaCodon {
    triplet: [DnaNucleotide; 3],
}

impl DnaCodon {
    pub fn transcribe(&self) -> RnaCodon {
        let map = {
            let mut map = HashMap::new();
            map.insert(&DnaNucleotide::A, RnaNucleotide::A);
            map.insert(&DnaNucleotide::C, RnaNucleotide::C);
            map.insert(&DnaNucleotide::T, RnaNucleotide::U);
            map.insert(&DnaNucleotide::G, RnaNucleotide::G);
            map
        };
        RnaCodon::from_triplet_arr(self.triplet.map(|b| *map.get(&b).unwrap()))
    }

    pub fn translate(&self) -> Eaa {
        Eaa::from(&self.transcribe())
    }
}

impl_codon_traits!(DnaNucleotide => DnaCodon);

#[cfg(test)]
mod tests {
    use super::DnaCodon;
    use super::DnaNucleotide::*;
    use crate::traits::*;

    #[test]
    fn test_dna_codon_from_string() -> anyhow::Result<()> {
        let codon = DnaCodon::try_from_str("ATG")?;
        assert_eq!(codon, [A, T, G].into());
        Ok(())
    }

    #[test]
    fn test_dna_codon_to_string() {
        let codon: DnaCodon = [A, T, G].into();
        assert_eq!(codon.to_string(), "ATG");
    }

    #[test]
    fn test_dna_codon_anticodon() {
        let codon: DnaCodon = [A, T, G].into();
        let anticodon = codon.anticodon();
        assert_eq!(anticodon, [T, A, C].into());
    }
}
