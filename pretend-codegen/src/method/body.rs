use super::BodyKind;
use crate::errors::{IError, IResult};
use crate::utils::parse_param_name;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::TraitItemMethod;

pub(crate) fn implement_body(method: &TraitItemMethod) -> IResult<TokenStream> {
    let kind = get_body(method)?;
    let implem = match kind {
        BodyKind::None => TokenStream::new(),
        BodyKind::Body => {
            quote! {
                let builder = {
                    let body = pretend::internal::IntoBody::into_body(body);
                    pretend::client::RequestBuilder::body(builder, body)?
                };
            }
        }
        BodyKind::Form => {
            quote! {
                let builder = pretend::client::RequestBuilder::form(builder, &form)?;
            }
        }
        BodyKind::Json => {
            quote! {
                let builder = pretend::client::RequestBuilder::json(builder, &json)?;
            }
        }
    };
    Ok(implem)
}

fn get_body(method: &TraitItemMethod) -> IResult<BodyKind> {
    let name = &method.sig.ident;

    let inputs = &method.sig.inputs;
    let mut iter = inputs
        .iter()
        .filter_map(parse_param_name)
        .filter_map(parse_body_kind);

    let first = iter.next();
    let second = iter.next();

    match (first, second) {
        (Some(_), Some(_)) => Err(IError::TooManyBodies(name.to_string())),
        (Some(result), None) => Ok(result),
        _ => Ok(BodyKind::None),
    }
}

fn parse_body_kind(ident: &Ident) -> Option<BodyKind> {
    if ident == "body" {
        Some(BodyKind::Body)
    } else if ident == "form" {
        Some(BodyKind::Form)
    } else if ident == "json" {
        Some(BodyKind::Json)
    } else {
        None
    }
}
