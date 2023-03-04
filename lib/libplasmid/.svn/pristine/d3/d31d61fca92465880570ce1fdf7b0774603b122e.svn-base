use std::{
    borrow::Borrow,
    collections::VecDeque,
    marker::PhantomData,
    ops::{Index, IndexMut},
};

use crate::{
    prelude::{IupacNucleotide, RestrictionEnzymes},
    traits::*,
};

use super::Annotation;

pub struct GeneticSequence<B, C>
where
    B: Nucleotide,
    C: Codon<B>,
{
    sequence: Vec<B>,
    annotations: Vec<Annotation>,
    phantom: PhantomData<C>,
}

impl<B, C> GeneticSequence<B, C>
where
    B: Nucleotide + TryFromLetter + ToLetter + ToIupac + Copy,
    C: Codon<B>,
{
    pub fn new() -> Self {
        Self {
            sequence: Vec::new(),
            annotations: Vec::new(),
            phantom: PhantomData,
        }
    }

    pub fn from_str<T>(s: T) -> anyhow::Result<GeneticSequence<B, C>>
    where
        T: AsRef<str>,
    {
        let mut builder = GeneticSequence::<B, C>::new();
        builder.push_base_str(s)?;
        Ok(builder)
    }
}

impl<B, C> Default for GeneticSequence<B, C>
where
    B: Nucleotide + TryFromLetter + ToLetter + ToIupac + Copy,
    C: Codon<B>,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<B, C> GeneticSequence<B, C>
where
    B: Nucleotide + TryFromLetter + ToLetter + ToIupac + Copy,
    C: Codon<B> + Sized,
{
    /// Append a nucleobase to the end of the sequence.
    ///
    /// # Examples
    /// ```rust
    /// use plasmid::prelude::{*, DnaNucleotide::*};
    ///
    /// let mut seq = DnaSequence::new();
    /// seq.push_base(A);
    ///
    /// assert_eq!(seq[0], A);
    /// ```
    pub fn push_base<T>(&mut self, base: T)
    where
        T: Borrow<B>,
    {
        self.sequence.push(*base.borrow())
    }

    /// Append a nucleobase string to the end of the sequence.
    ///
    /// # Examples
    /// ```rust
    /// use plasmid::prelude::{*, DnaNucleotide::*};
    ///
    /// let mut seq = DnaSequence::new();
    /// seq.push_base_str("AGT");
    ///
    /// assert_eq!(seq.to_string(), "AGT");
    /// ```
    pub fn push_base_str<T>(&mut self, s: T) -> anyhow::Result<()>
    where
        T: AsRef<str>,
    {
        for c in s.as_ref().chars() {
            let base = B::try_from_letter(c)?;
            self.push_base(base)
        }
        Ok(())
    }

    /// Append a codon to the end of the sequence.
    ///
    /// # Examples
    /// ```rust
    /// use plasmid::prelude::{*, DnaNucleotide::*};
    ///
    /// let mut seq = DnaSequence::new();
    /// let codon: DnaCodon = [A, G, T].into();
    /// seq.push_codon(&codon);
    ///
    /// assert_eq!(seq.last_codon(), Some([A, G, T].into()));
    /// ```
    pub fn push_codon<T>(&mut self, codon: T)
    where
        T: Borrow<C>,
    {
        self.sequence.extend(codon.borrow().to_triplet_arr())
    }

    /// Remove the last nucleobase from the sequence and return it, or `None` if it is empty.
    ///
    /// # Examples
    /// ```rust
    /// use plasmid::prelude::{*, DnaNucleotide::*};
    ///
    /// let mut seq = DnaSequence::from_str("AGTCCT").unwrap();
    /// let base = seq.pop_base().unwrap();
    ///
    /// assert_eq!(base, T);
    /// ```
    pub fn pop_base(&mut self) -> Option<B> {
        self.sequence.pop()
    }

    /// Return the last codon from the sequence, or `None` if there are none.
    /// The function will return the last proper codon in the sequence.
    ///
    /// Use `#last_codon_unsafe` if you need a codon from the last three nucleobases.
    ///
    /// # Examples
    /// ```rust
    /// use plasmid::prelude::{*, DnaNucleotide::*};
    ///
    /// let mut seq1 = DnaSequence::from_str("AGTAA").unwrap();
    /// let seq1_codon = seq1.last_codon().unwrap(); // AGT
    ///
    /// assert_eq!(seq1_codon, [A, G, T].into());
    ///
    /// let mut seq2 = DnaSequence::from_str("AA").unwrap();
    /// let seq2_codon = seq2.last_codon(); // None
    ///
    /// assert!(seq2_codon.is_none());
    /// ```
    pub fn last_codon(&self) -> Option<C> {
        self.codon_iter().last()
    }

    /// Return the last codon from the sequence, or `None` if there are none.
    /// This function will build a codon from the last nucleotide triplet.
    ///
    /// # Examples
    /// ```rust
    /// use plasmid::prelude::{*, DnaNucleotide::*};
    ///
    /// let mut seq1 = DnaSequence::from_str("AGTAA").unwrap();
    /// let seq1_codon = seq1.last_codon_unsafe().unwrap(); // TAA
    ///
    /// assert_eq!(seq1_codon, [T, A, A].into());
    ///
    /// let mut seq2 = DnaSequence::from_str("AA").unwrap();
    /// let seq2_codon = seq2.last_codon_unsafe(); // None
    ///
    /// assert!(seq2_codon.is_none());
    /// ```
    pub fn last_codon_unsafe(&self) -> Option<C> {
        let seq: [B; 3] = self
            .sequence
            .iter()
            .rev()
            .take(3)
            .rev()
            .cloned()
            .collect::<Vec<B>>()
            .try_into()
            .ok()?;
        Some(C::from_triplet_arr(seq))
    }

    /// An iterator over the nucleotides of a genetic sequence.
    ///
    /// # Examples
    /// ```
    /// use plasmid::prelude::{*, DnaNucleotide::*};
    ///
    /// let seq = DnaSequence::from_str("TGATCC").unwrap();
    /// let nucleotides = seq.nucleotide_iter().map(|&x| x).collect::<Vec<_>>();
    ///
    /// assert_eq!(nucleotides, [T, G, A, T, C, C])
    /// ```
    pub fn nucleotide_iter(&self) -> std::slice::Iter<B> {
        self.sequence.iter()
    }

    /// An iterator over the codons of a genetic sequence.
    ///
    /// # Examples
    /// ```rust
    /// use plasmid::prelude::*;
    ///
    /// let seq = DnaSequence::from_str("TGATCC").unwrap();
    /// for codon in seq.codon_iter() {
    ///     println!("{:?}", codon);
    /// }
    /// ```
    pub fn codon_iter(&self) -> impl Iterator<Item = C> + '_ {
        self.sequence
            .chunks_exact(3)
            .map(|chunk| C::from_triplet_arr(chunk.try_into().unwrap()))
    }

    /// An iterator over the IUPAC sequence.
    ///
    /// # Examples
    /// ```rust
    /// use plasmid::prelude::{*, IupacNucleotide::*};
    ///
    /// let seq = DnaSequence::from_str("TGATCC").unwrap();
    /// assert_eq!(&seq.iupac_iter().collect::<Vec<_>>(), &[T, G, A, T, C, C])
    /// ```
    pub fn iupac_iter(&self) -> impl Iterator<Item = IupacNucleotide> + '_ {
        self.sequence.iter().map(|n| n.to_iupac())
    }

    /// An iterator over the associated annotations.
    ///
    /// # Examples
    /// ```rust
    /// use plasmid::prelude::*;
    ///
    /// let mut seq = DnaSequence::from_str("ATGTTC").unwrap();
    /// seq.as_mut_annotations().push(Annotation::new(0, 2, None, "Start Codon"));
    /// assert_eq!(seq.annotation_iter().next().unwrap().text, "Start Codon");
    /// ```
    pub fn annotation_iter(&self) -> std::slice::Iter<Annotation> {
        self.annotations.iter()
    }

    /// An iterator over the associated annotations.
    ///
    /// # Examples
    /// ```rust
    /// use plasmid::prelude::*;
    ///
    /// let mut seq = DnaSequence::from_str("ATGTTC").unwrap();
    /// let ann = Annotation::new(0, 2, None, "Start Codon");
    /// seq.as_mut_annotations().push(ann);
    /// assert_eq!(seq.annotation_iter().next().unwrap().text, "Start Codon");
    /// ```
    pub fn as_annotations(&self) -> &[Annotation] {
        &self.annotations
    }

    pub fn as_mut_annotations(&mut self) -> &mut Vec<Annotation> {
        &mut self.annotations
    }

    /// An iterator over the nucleotides of a genetic sequence.
    ///
    /// # Examples
    /// ```
    /// use plasmid::prelude::{*, DnaNucleotide::*};
    ///
    /// let seq = DnaSequence::from_str("TGATCC").unwrap();
    /// let nucleotides = seq.as_nucleotides();
    ///
    /// assert_eq!(nucleotides, [T, G, A, T, C, C])
    /// ```
    pub fn as_nucleotides(&self) -> &[B] {
        &self.sequence
    }

    /// Convert a genetic sequence to a Vec of its anti-nucleotides.
    ///
    /// # Examples
    /// ```
    /// use plasmid::prelude::{*, DnaNucleotide::*};
    ///
    /// let seq = DnaSequence::from_str("TGATCC").unwrap();
    /// let nucleotides = seq.as_reverse_complement();
    ///
    /// assert_eq!(nucleotides, [A, C, T, A, G, G])
    /// ```
    pub fn as_reverse_complement(&self) -> Vec<B> {
        self.sequence
            .iter()
            .map(|b| b.complement())
            .collect::<Vec<_>>()
    }

    /// Convert a genetic sequence to a Vec of its codons.
    ///
    /// # Examples
    /// ```rust
    /// use plasmid::prelude::{*, DnaNucleotide::*};
    ///
    /// let seq = DnaSequence::from_str("TGATCC").unwrap();
    ///
    /// assert_eq!(seq.as_codons(), [[T, G, A].into(), [T, C, C].into()])
    /// ```
    pub fn as_codons(&self) -> Vec<C> {
        self.codon_iter().collect()
    }

    /// Return the nucleotide sequence as a string.
    ///
    /// # Example
    /// ```rust
    /// use plasmid::prelude::*;
    ///
    /// let seq = DnaSequence::from_str("ATGTTC").unwrap();
    /// assert_eq!(seq.to_nucleotide_string(), "ATGTTC");
    /// ```
    pub fn to_nucleotide_string(&self) -> String {
        self.as_nucleotides()
            .iter()
            .map(|b| b.to_letter())
            .collect()
    }

    /// Return the reverse complement of the nucleotide sequence as a string.
    ///
    /// # Example
    /// ```rust
    /// use plasmid::prelude::*;
    ///
    /// let seq = DnaSequence::from_str("ATGTTC").unwrap();
    /// assert_eq!(seq.to_reverse_complement_string(), "TACAAG");
    /// ```
    pub fn to_reverse_complement_string(&self) -> String {
        self.as_reverse_complement()
            .iter()
            .map(|b| b.to_letter())
            .collect()
    }

    /// Count guanine and cytosine nucleotides
    ///
    /// # Example
    /// ```rust
    /// use plasmid::prelude::*;
    ///
    /// let seq = DnaSequence::from_str("ATGTTC").unwrap();
    /// assert_eq!(seq.gc_count(), 2);
    /// ```
    pub fn gc_count(&self) -> usize {
        use IupacNucleotide::*;
        let nucleotides = [G, C];
        self.iupac_iter()
            .filter(|n| nucleotides.contains(n))
            .count()
    }

    /// Count adenine and thymine/uracil nucleotides
    ///
    /// # Example
    /// ```rust
    /// use plasmid::prelude::*;
    ///
    /// let seq = DnaSequence::from_str("ATGTTC").unwrap();
    /// assert_eq!(seq.at_count(), 4);
    /// ```
    pub fn at_count(&self) -> usize {
        use IupacNucleotide::*;
        let nucleotides = [A, T];
        self.iupac_iter()
            .filter(|n| nucleotides.contains(n))
            .count()
    }

    /// Compute guanine-cytosine ratio
    ///
    /// # Example
    /// ```rust
    /// use plasmid::prelude::*;
    ///
    /// let seq = DnaSequence::from_str("ATGTTC").unwrap();
    /// let expected = 2_f32 / 6_f32;
    /// assert!((seq.gc_ratio() - expected).abs() <= std::f32::EPSILON);
    /// ```
    pub fn gc_ratio(&self) -> f32 {
        self.gc_count() as f32 / self.sequence.len() as f32
    }

    /// Compute adenine-thymine ratio
    ///
    /// # Example
    /// ```rust
    /// use plasmid::prelude::*;
    ///
    /// let seq = DnaSequence::from_str("ATGTTC").unwrap();
    /// let expected = 4_f32 / 6_f32;
    /// assert!((seq.at_ratio() - expected).abs() <= std::f32::EPSILON);
    /// ```
    pub fn at_ratio(&self) -> f32 {
        self.at_count() as f32 / self.sequence.len() as f32
    }

    /// Compute adenine-thymine/uracil to guanine-cytosine ratio
    ///
    /// # Example
    /// ```rust
    /// use plasmid::prelude::*;
    ///
    /// let seq = DnaSequence::from_str("ATGTTC").unwrap();
    /// let expected = 4_f32 / 2_f32;
    /// assert!((seq.at_gc_ratio() - expected).abs() <= std::f32::EPSILON);
    /// ```
    pub fn at_gc_ratio(&self) -> f32 {
        self.at_count() as f32 / self.gc_count() as f32
    }

    /// Annotate known restriction enzymes.
    ///
    /// The algorithm will iterate over the sequence multiple times
    /// and try to find cut sites of known restriction enzymes.
    ///
    /// All detected cut sites will be annotated with their corresponding
    /// start-, stop-, and cut-positions. The cut position will be stored
    /// inside of the `needle` variable of the `Annotation`.
    ///
    /// This function is relatively slow, don't run it on every insertion/deletion.
    ///
    /// # Examples
    /// ```rust
    /// use plasmid::prelude::*;
    ///
    /// let mut seq = DnaSequence::from_str("ATGTTCCATATGTCTCGT").unwrap();
    /// seq.annotate_restriction_enzymes(); // should find NdeI: CA/TATG
    /// let ann = seq.as_annotations().first().unwrap();
    /// assert_eq!(ann.text, "NdeI");
    /// assert_eq!(ann.start, 5);
    /// assert_eq!(ann.needle, Some(7));
    /// assert_eq!(ann.end, 11);
    /// ```
    pub fn annotate_restriction_enzymes(&mut self) {
        let own_nucleotides = self.as_nucleotides();
        let mut annotations: Vec<Annotation> = Vec::new();
        for enzyme in RestrictionEnzymes.iter() {
            let mut enzyme_seq = enzyme.before.clone();
            enzyme_seq.extend(&enzyme.after);
            let mut nucleotides: VecDeque<&B> = VecDeque::new();
            for (own_index, own_nucleotide) in own_nucleotides.iter().enumerate() {
                if nucleotides.len() == enzyme_seq.len() {
                    nucleotides.pop_front();
                }
                nucleotides.push_back(own_nucleotide);
                if nucleotides.len() == enzyme_seq.len() {
                    let seq_matches = enzyme_seq
                        .iter()
                        .zip(nucleotides.iter().map(|n| n.to_iupac()))
                        .all(|(n, m)| n.matches(&m));
                    if seq_matches {
                        let needle = own_index - nucleotides.len() + enzyme.before.len();
                        let ann = Annotation::new_from_restriction_enzyme(
                            own_index - nucleotides.len(),
                            own_index,
                            Some(needle),
                            enzyme,
                        );
                        annotations.push(ann);
                    }
                }
            }
        }
        self.annotations.extend(annotations);
    }
}

impl<B, C> ToString for GeneticSequence<B, C>
where
    B: Nucleotide + ToLetter,
    C: Codon<B>,
{
    fn to_string(&self) -> String {
        self.sequence.iter().map(|b| b.to_letter()).collect()
    }
}

impl<B, C> Index<usize> for GeneticSequence<B, C>
where
    B: Nucleotide,
    C: Codon<B>,
{
    type Output = B;

    fn index(&self, index: usize) -> &Self::Output {
        &self.sequence[index]
    }
}

impl<B, C> IndexMut<usize> for GeneticSequence<B, C>
where
    B: Nucleotide,
    C: Codon<B>,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.sequence[index]
    }
}

impl<B, C> NucleotideSequence for GeneticSequence<B, C>
where
    B: Nucleotide + ToIupac,
    C: Codon<B>,
{
    fn matches<T>(&self, seq: &[T]) -> bool
    where
        T: ToIupac,
    {
        self.sequence
            .iter()
            .zip(seq)
            .all(|(a, b)| a.to_iupac().matches(&b.to_iupac()))
    }
}

#[cfg(test)]
mod tests {
    use crate::seq::{DnaSequence, RnaSequence};

    #[test]
    fn test_rna_sequence_from_str() {
        use crate::rna::RnaNucleotide::*;

        let seq = RnaSequence::from_str("AUGUGAUGAAAGCAUAUGACUAAA");
        assert!(seq.is_ok());
        let seq = seq.unwrap();
        let codons = seq.as_codons();
        assert_eq!(
            codons,
            &[
                [A, U, G].into(), // Met
                [U, G, A].into(), // Ter
                [U, G, A].into(), // Ter
                [A, A, G].into(), // Lys
                [C, A, U].into(), // His
                [A, U, G].into(), // Met
                [A, C, U].into(), // Thr
                [A, A, A].into(), // Lys
            ]
        );
    }

    #[test]
    pub fn test_annotate_restriction_enzymes() {
        let mut seq = DnaSequence::from_str("AAAACATATGAAAA").unwrap();
        seq.annotate_restriction_enzymes();
        assert_eq!(seq.annotations.len(), 1);
        let ann = seq.annotations.first().unwrap();
        assert_eq!(ann.start, 3);
        assert_eq!(ann.needle, Some(5));
        assert_eq!(ann.end, 9);
        assert_eq!(ann.text, "NdeI");
    }
}
