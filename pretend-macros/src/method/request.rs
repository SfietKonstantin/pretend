use crate::errors::{IError, IResult};
use crate::utils::parse_name_value_attr;
use proc_macro2::Ident;
use std::iter::FromIterator;
use syn::{Attribute, TraitItemMethod};

pub(crate) fn get_request(method: &TraitItemMethod) -> IResult<(String, String)> {
    let name = &method.sig.ident;
    let attrs = &method.attrs;

    let mut iter = attrs
        .iter()
        .filter_map(|attr| parse_request_attr(attr, name));

    let first = iter.next();
    let second = iter.next();

    match (first, second) {
        (Some(_), Some(_)) => Err(IError::TooManyRequests(name.to_string())),
        (Some(result), None) => result,
        _ => Err(IError::MissingRequest(name.to_string())),
    }
}

fn parse_request_attr(attr: &Attribute, name: &Ident) -> Option<IResult<(String, String)>> {
    let name_values = parse_name_value_attr("request", attr)?;
    let request = name_values.into_iter().collect::<Request>();
    Some(request.into_result(name))
}

struct Request {
    method: Option<String>,
    path: Option<String>,
}

impl Request {
    fn into_result(self, name: &Ident) -> IResult<(String, String)> {
        let method = self
            .method
            .ok_or_else(|| IError::MissingMethod(name.to_string()))?;
        let path = self
            .path
            .ok_or_else(|| IError::MissingPath(name.to_string()))?;
        Ok((method, path))
    }
}

impl FromIterator<(String, String)> for Request {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (String, String)>,
    {
        let mut method = None;
        let mut path = None;

        for (key, value) in iter {
            match key.as_str() {
                "method" => method = Some(value),
                "path" => path = Some(value),
                _ => {}
            }
        }

        Request { method, path }
    }
}
