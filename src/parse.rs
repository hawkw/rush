pub trait ParseInto<'a, T: Parse<'a>> {
    fn parse_into(&'a self) -> Result<T, T::Error>;
}

pub trait Parse<'a>: Sized {
    type Error;
    fn parse_from(input: &'a str) -> Result<Self, Self::Error>;
}

impl<'a, T, P: 'a> ParseInto<'a, T> for P
where
    P: AsRef<str>,
    T: Parse<'a>,
{
    fn parse_into(&'a self) -> Result<T, T::Error> {
        T::parse_from(self.as_ref())
    }
}
