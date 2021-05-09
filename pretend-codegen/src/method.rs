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
use crate::errors::{IError, IResult};
use crate::format::format;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{TraitItem, TraitItemMethod};

pub(crate) enum BodyKind {
    None,
    Body,
    Form,
    Json,
}

pub(crate) fn implement_trait_item(item: &TraitItem) -> IResult<TokenStream> {
    match item {
        TraitItem::Method(method) => implement_method(method),
        _ => Err(IError::UnsupportedTraitItem),
    }
}

fn implement_method(method: &TraitItemMethod) -> IResult<TokenStream> {
    check_no_generics(method)?;
    check_correct_receiver(method)?;

    let query = implement_query(method);
    let body = implement_body(method)?;
    let headers = implement_headers(method);

    let sig = &method.sig;
    let (method, path) = get_request(method)?;
    let method = Ident::new(&method, Span::call_site());
    let path = format(path, "path");

    Ok(quote! {
        #sig {
            let method = pretend::Method::#method;
            #path
            #headers

            let support = pretend::internal::MacroSupport::new(self);
            let builder = support.request(method, path, headers)?;
            #query
            #body

            let request = pretend::client::RequestBuilder::build(builder);
            let response = support.execute(request).await?;
            pretend::internal::MacroResponseSupport::into_response(response).await
        }
    })
}
