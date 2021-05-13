use quote::ToTokens;

pub(crate) struct WithTokens<'a, V, T>
where
    T: ToTokens,
{
    pub(crate) value: V,
    pub(crate) tokens: &'a T,
}

impl<'a, V, T> WithTokens<'a, V, T>
where
    T: ToTokens,
{
    pub(crate) fn new(value: V, tokens: &'a T) -> Self {
        WithTokens { value, tokens }
    }
}
