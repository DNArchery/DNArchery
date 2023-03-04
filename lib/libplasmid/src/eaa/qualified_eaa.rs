use std::ops::Deref;

use super::Eaa;
use crate::rna::RnaCodon;

/// Fully qualified essential amino acid with rna codon sequence
#[derive(Debug, PartialEq, Eq)]
pub struct QualifiedEaa {
    /// The essential amino acid
    pub eaa: Eaa,
    /// The corresponding rna codon
    pub codon: RnaCodon,
}

impl Deref for QualifiedEaa {
    type Target = Eaa;

    fn deref(&self) -> &Eaa {
        &self.eaa
    }
}

impl Into<Eaa> for QualifiedEaa {
    fn into(self) -> Eaa {
        self.eaa
    }
}

impl Into<RnaCodon> for QualifiedEaa {
    fn into(self) -> RnaCodon {
        self.codon
    }
}

impl From<RnaCodon> for QualifiedEaa {
    fn from(codon: RnaCodon) -> QualifiedEaa {
        let eaa = Eaa::from(&codon);
        QualifiedEaa { eaa, codon }
    }
}
