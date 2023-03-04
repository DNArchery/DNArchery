use super::{RnaCodon, RnaNucleotide};

#[derive(Debug, PartialEq, Eq)]
pub struct RnaPartialCodon {
    bases: Vec<RnaNucleotide>,
}

impl RnaPartialCodon {
    pub fn new() -> Self {
        RnaPartialCodon {
            bases: Vec::with_capacity(3),
        }
    }

    pub fn from_slice(slice: &[RnaNucleotide]) -> RnaPartialCodon {
        let mut pcodon = RnaPartialCodon::new();
        for base in slice[0..(slice.len().min(3))].iter() {
            pcodon.push(*base).unwrap();
        }
        pcodon
    }
}

impl Default for RnaPartialCodon {
    fn default() -> Self {
        Self::new()
    }
}

impl RnaPartialCodon {
    pub fn push(&mut self, item: RnaNucleotide) -> anyhow::Result<()> {
        if self.nucleobase_count() == 3 {
            return Err(anyhow!(
                "Cannot push more than two bases into a partial codon"
            ));
        } else {
            self.bases.push(item);
            Ok(())
        }
    }

    pub fn pop(&mut self) -> Option<RnaNucleotide> {
        self.bases.pop()
    }

    pub fn nucleobase_count(&self) -> usize {
        self.bases.len()
    }

    pub fn to_codon(&self) -> Option<RnaCodon> {
        let arr = [
            *self.bases.get(0)?,
            *self.bases.get(1)?,
            *self.bases.get(2)?,
        ];
        Some(arr.into())
    }
}

#[cfg(test)]
mod tests {
    use super::RnaPartialCodon;

    #[test]
    fn test_rna_partial_codon_new() {
        let builder = RnaPartialCodon::new();
        assert!(builder.to_codon().is_none());
    }

    #[test]
    fn test_rna_partial_codon_nucleobase_count() {
        use crate::rna::RnaNucleotide::*;
        let pcodon = RnaPartialCodon::from_slice(&[A, U]);
        assert_eq!(pcodon.nucleobase_count(), 2);
    }

    #[test]
    fn test_rna_partial_codon_to_codon() {
        use crate::rna::RnaNucleotide::*;
        let pcodon = RnaPartialCodon::from_slice(&[A, U, G]);
        let codon = pcodon.to_codon();
        assert!(codon.is_some());
        if let Some(codon) = codon {
            assert_eq!(codon, [A, U, G].into());
        }
    }
}
