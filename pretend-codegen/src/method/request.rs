use crate::errors::{
    ErrorsExt, INVALID_REQUEST, MISSING_REQUEST, TOO_MANY_REQUESTS, TOO_MANY_REQUESTS_HINT,
};
use crate::method::parse_request_attr;
use crate::utils::Single;
use syn::{Error, Result, TraitItemMethod};

pub(crate) fn get_request(method: &TraitItemMethod) -> Result<(String, String)> {
    let attrs = &method.attrs;
    let single = attrs
        .iter()
        .filter_map(parse_request_attr)
        .collect::<Single<_>>();

    match single {
        Single::None => Err(Error::new_spanned(method, MISSING_REQUEST)),
        Single::Single(item) => {
            let value = item.value;
            let tokens = item.tokens;
            value.ok_or_else(|| Error::new_spanned(tokens, INVALID_REQUEST))
        }
        Single::TooMany(requests) => {
            let errors = requests
                .into_iter()
                .map(|item| Error::new_spanned(item.tokens, TOO_MANY_REQUESTS_HINT))
                .collect::<Vec<_>>();

            errors.into_result(|| Error::new_spanned(method, TOO_MANY_REQUESTS))
        }
    }
}
