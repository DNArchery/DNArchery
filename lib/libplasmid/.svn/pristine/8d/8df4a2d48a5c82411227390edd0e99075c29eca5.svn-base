pub trait FromStr<'a, T>
where
    T: AsRef<str>,
{
    fn from_str(s: T) -> Self;
}

pub trait TryFromStr<'a, T>
where
    Self: Sized,
    T: AsRef<str>,
{
    fn try_from_str(s: T) -> anyhow::Result<Self>;
}
