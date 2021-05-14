use std::iter::FromIterator;

pub(crate) enum Single<T> {
    None,
    Single(T),
    TooMany(Vec<T>),
}

impl<T> FromIterator<T> for Single<T> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let mut iter = iter.into_iter();

        let item1 = iter.next();
        let item2 = iter.next();

        match (item1, item2) {
            (None, None) => Self::None,
            (Some(item), None) => Self::Single(item),
            (item1, item2) => {
                let iter1 = item1.into_iter();
                let iter2 = item2.into_iter();
                let all = iter1.chain(iter2).chain(iter).collect();
                Self::TooMany(all)
            }
        }
    }
}
