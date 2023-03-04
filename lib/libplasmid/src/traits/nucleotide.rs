pub trait Nucleotide {
    fn complement(&self) -> Self;
    fn base_pair(&self) -> (Self, Self)
    where
        Self: Sized + Copy,
    {
        (*self, self.complement())
    }
}
