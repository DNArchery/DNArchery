pub trait Export {
    type Output;

    fn export(&self) -> Self::Output;
}
