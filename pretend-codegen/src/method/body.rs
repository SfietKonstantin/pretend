use super::BodyKind;
use crate::errors::{ErrorsExt, TOO_MANY_BODIES, TOO_MANY_BODIES_HINT};
use crate::utils::{parse_param_name, Single, WithTokens};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Error, Result, TraitItemMethod};

pub(crate) fn implement_body(method: &TraitItemMethod) -> Result<TokenStream> {
    let kind = get_body(method)?;
    let implem = match kind {
        BodyKind::None => {
            quote! {
                let body = pretend::internal::Body::<()>::None;
            }
        }
        BodyKind::Body => {
            quote! {
                let body = pretend::internal::Body::<()>::Raw(pretend::client::Bytes::from(body));
            }
        }
        BodyKind::Form => {
            quote! {
                let body = pretend::internal::Body::Form(&form);
            }
        }
        BodyKind::Json => {
            quote! {
                let body = pretend::internal::Body::Json(&json);
            }
        }
    };
    Ok(implem)
}

fn get_body(method: &TraitItemMethod) -> Result<BodyKind> {
    let inputs = &method.sig.inputs;
    let single = inputs
        .iter()
        .filter_map(parse_param_name)
        .filter_map(parse_body_kind)
        .collect::<Single<_>>();

    match single {
        Single::None => Ok(BodyKind::None),
        Single::Single(item) => Ok(item.value),
        Single::TooMany(bodies) => {
            let errors = bodies
                .into_iter()
                .map(|item| Error::new_spanned(item.tokens, TOO_MANY_BODIES_HINT))
                .collect::<Vec<_>>();

            errors.into_result(|| Error::new_spanned(&method.sig, TOO_MANY_BODIES))
        }
    }
}

fn parse_body_kind(ident: &Ident) -> Option<WithTokens<BodyKind, Ident>> {
    if ident == "body" {
        Some(WithTokens::new(BodyKind::Body, ident))
    } else if ident == "form" {
        Some(WithTokens::new(BodyKind::Form, ident))
    } else if ident == "json" {
        Some(WithTokens::new(BodyKind::Json, ident))
    } else {
        None
    }
}
