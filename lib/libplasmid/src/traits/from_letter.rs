pub trait FromLetter {
    fn from_letter(c: char) -> Self;
}

pub trait TryFromLetter
where
    Self: Sized,
{
    fn try_from_letter(c: char) -> anyhow::Result<Self>;
}
