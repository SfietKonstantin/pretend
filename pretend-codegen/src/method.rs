mod body;
mod checks;
mod headers;
mod query;
mod request;

use self::body::implement_body;
use self::checks::{check_correct_receiver, check_no_generics};
use self::headers::implement_headers;
use self::query::implement_query;
use self::request::get_request;
use crate::errors::UNSUPPORTED_TRAIT_ITEM;
use crate::format::format;
use crate::ClientKind;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{Error, Result, TraitItem, TraitItemMethod};

pub(crate) enum BodyKind {
    None,
    Body,
    Form,
    Json,
}

pub(crate) fn implement_trait_item(item: &TraitItem, kind: &ClientKind) -> Result<TokenStream> {
    match item {
        TraitItem::Method(method) => implement_method(method, kind),
        _ => Err(Error::new_spanned(item, UNSUPPORTED_TRAIT_ITEM)),
    }
}

fn implement_method(method: &TraitItemMethod, kind: &ClientKind) -> Result<TokenStream> {
    check_no_generics(method)?;
    check_correct_receiver(method)?;

    let query = implement_query(method);
    let body = implement_body(method)?;
    let headers = implement_headers(method)?;

    let sig = &method.sig;
    let (method, path) = get_request(method)?;
    let method = Ident::new(&method, Span::call_site());
    let path = format(path, "path");

    let execute_request = match kind {
        ClientKind::Async => quote! {
            support.request(method, url, headers, body).await
        },
        ClientKind::AsyncLocal => quote! {
            support.request_local(method, url, headers, body).await
        },
        ClientKind::Blocking => quote! {
            support.request_blocking(method, url, headers, body)
        },
    };

    Ok(quote! {
        #sig {
            let method = pretend::client::Method::#method;
            #path
            #headers
            #body

            let support = pretend::internal::MacroSupport::new(self);
            let url = support.create_url(path)?;
            #query

            let response = #execute_request ?;
            pretend::internal::IntoResponse::into_response(response)
        }
    })
}
