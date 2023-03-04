use super::Import;
use crate::{
    prelude::{DnaNucleotide, Eaa, RnaNucleotide},
    traits::TryFromLetter,
    uni::IupacNucleotide,
};

pub struct FastaFile {
    pub description: String,
    pub sequence: String,
}

pub struct TypedFastaFile<Item> {
    pub description: String,
    pub sequence: Vec<Item>,
}

/// A FASTA file containing an IUB/IUPAC DNA sequence.
pub type FastaIupacFile = TypedFastaFile<IupacNucleotide>;

/// A FASTA file containing a peptide sequence.
pub type FastaEaaFile = TypedFastaFile<Eaa>;

impl Import for FastaFile {
    type Output = Self;

    /// Import a FASTA file from a string.
    ///
    /// # Example
    /// ```rust
    /// use plasmid::prelude::*;
    /// let fasta = FastaFile::import(">test sequence\nATGAACGCGTCC").unwrap();
    /// assert_eq!(fasta.description, "test sequence");
    /// assert_eq!(fasta.sequence, "ATGAACGCGTCC");
    /// ```
    fn import<S>(s: S) -> anyhow::Result<Self::Output>
    where
        S: AsRef<str>,
    {
        let lines = s.as_ref().lines();
        let mut description = String::new();
        let mut sequence = String::new();
        for line in lines {
            if let Some(stripped) = line.strip_prefix('>') {
                if !description.is_empty() {
                    bail!("Invalid FASTA file: multiple sequences found")
                }
                description = stripped.to_string();
            } else {
                sequence.push_str(line);
            }
        }
        Ok(FastaFile {
            description,
            sequence,
        })
    }
}

impl<Item> Import for TypedFastaFile<Item>
where
    Item: TryFromLetter,
{
    type Output = Self;

    /// Import a FASTA file from a string.
    ///
    /// # Example
    /// ```rust
    /// use plasmid::prelude::{*, IupacNucleotide::*};
    /// let fasta = FastaIupacFile::import(">test sequence\nATGAACGCGTNN").unwrap();
    /// assert_eq!(fasta.description, "test sequence");
    /// assert_eq!(fasta.sequence, [A, T, G, A, A, C, G, C, G, T, N, N]);
    /// ```
    fn import<S>(s: S) -> anyhow::Result<Self::Output>
    where
        S: AsRef<str>,
    {
        let fasta = FastaFile::import(s)?;
        let sequence = fasta.as_sequence::<Item>()?;
        let description = fasta.description;
        Ok(TypedFastaFile {
            description,
            sequence,
        })
    }
}

impl FastaFile {
    fn as_sequence<S>(&self) -> anyhow::Result<Vec<S>>
    where
        S: TryFromLetter,
    {
        let mut sequence = Vec::with_capacity(self.sequence.len());
        for char in self.sequence.chars() {
            sequence.push(TryFromLetter::try_from_letter(char)?);
        }
        Ok(sequence)
    }

    pub fn as_iupac_sequence(&self) -> anyhow::Result<Vec<IupacNucleotide>> {
        self.as_sequence()
    }

    pub fn as_dna_sequence(&self) -> anyhow::Result<Vec<DnaNucleotide>> {
        self.as_sequence()
    }

    pub fn as_rna_sequence(&self) -> anyhow::Result<Vec<RnaNucleotide>> {
        self.as_sequence()
    }

    pub fn as_amino_acid_sequence(&self) -> anyhow::Result<Vec<Eaa>> {
        self.as_sequence()
    }

    pub fn is_amino_acid_sequence(&self) -> bool {
        self.sequence.chars().all(|c| Eaa::all_as_str().contains(c))
    }

    pub fn is_iupac_sequence(&self) -> bool {
        self.sequence
            .chars()
            .all(|c| IupacNucleotide::all_as_str().contains(c))
    }

    pub fn is_dna_sequence(&self) -> bool {
        self.sequence
            .chars()
            .all(|c| DnaNucleotide::all_as_str().contains(c))
    }

    pub fn is_rna_sequence(&self) -> bool {
        self.sequence
            .chars()
            .all(|c| RnaNucleotide::all_as_str().contains(c))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        imp::FastaEaaFile,
        prelude::{DnaNucleotide, Eaa, IupacNucleotide, RnaNucleotide},
    };

    use super::{FastaFile, Import, TypedFastaFile};

    #[test]
    fn test_fasta_import_valid() -> anyhow::Result<()> {
        let fasta = FastaFile::import(">test\nATGC")?;
        assert_eq!(fasta.description, "test");
        assert_eq!(fasta.sequence, "ATGC");
        Ok(())
    }

    #[test]
    #[should_panic]
    fn test_fasta_import_multiple_descriptions() {
        FastaFile::import(">test\n>test\nATGC").unwrap();
    }

    #[test]
    fn test_fasta_file_as_iupac_sequence_valid() -> anyhow::Result<()> {
        use crate::uni::IupacNucleotide::*;
        let fasta = FastaFile::import(">test\nATGCNN")?;
        assert!(fasta.is_iupac_sequence());
        let sequence = fasta.as_iupac_sequence()?;
        assert_eq!(sequence, [A, T, G, C, N, N]);
        Ok(())
    }

    #[test]
    fn test_typed_fasta_file_import_iupac_valid() -> anyhow::Result<()> {
        use crate::uni::IupacNucleotide::*;
        let fasta = TypedFastaFile::<IupacNucleotide>::import(">test\nATGCNN")?;
        assert_eq!(fasta.sequence, [A, T, G, C, N, N]);
        Ok(())
    }

    #[test]
    fn test_typed_fasta_file_import_dna_valid() -> anyhow::Result<()> {
        use crate::dna::DnaNucleotide::*;
        let fasta = TypedFastaFile::<DnaNucleotide>::import(">test\nATGCTC")?;
        assert_eq!(fasta.sequence, [A, T, G, C, T, C]);
        Ok(())
    }

    #[test]
    fn test_typed_fasta_file_import_rna_valid() -> anyhow::Result<()> {
        use crate::rna::RnaNucleotide::*;
        let fasta = TypedFastaFile::<RnaNucleotide>::import(">test\nAUGCUC")?;
        assert_eq!(fasta.sequence, [A, U, G, C, U, C]);
        Ok(())
    }

    #[test]
    fn test_typed_fasta_file_import_eaa_valid() -> anyhow::Result<()> {
        use crate::eaa::Eaa::*;
        let fasta = TypedFastaFile::<Eaa>::import(">test\nMGA")?;
        assert_eq!(fasta.sequence, [Met, Gly, Ala]);
        Ok(())
    }

    #[test]
    fn test_typed_fasta_file_import_complex_peptide_sequence_valid() -> anyhow::Result<()> {
        let fasta = FastaEaaFile::import(
            r#"
>gi|186681228|ref|YP_001864424.1| phycoerythrobilin:ferredoxin oxidoreductase
MNSERSDVTLYQPFLDYAIAYMRSRLDLEPYPIPTGFESNSAVVGKGKNQEEVVTTSYAFQTAKLRQIRA
AHVQGGNSLQVLNFVIFPHLNYDLPFFGADLVTLPGGHLIALDMQPLFRDDSAYQAKYTEPILPIFHAHQ
QHLSWGGDFPEEAQPFFSPAFLWTRPQETAVVETQVFAAFKDYLKAYLDFVEQAEAVTDSQNLVAIKQAQ
LRYLRYRAEKDPARGMFKRFYGAEWTEEYIHGFLFDLERKLTVVK
        "#
            .trim(),
        )?;
        assert_eq!(
            fasta.description,
            "gi|186681228|ref|YP_001864424.1| phycoerythrobilin:ferredoxin oxidoreductase"
        );
        Ok(())
    }
}
