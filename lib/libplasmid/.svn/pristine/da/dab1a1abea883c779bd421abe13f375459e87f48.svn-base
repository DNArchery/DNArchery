use super::ToIupac;

pub trait NucleotideSequence {
    fn matches<T>(&self, seq: &[T]) -> bool
    where
        T: ToIupac;
}
