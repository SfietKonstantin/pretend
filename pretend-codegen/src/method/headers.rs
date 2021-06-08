use crate::errors::{Report, INVALID_HEADER, METHOD_FAILURE};
use crate::format::format;
use crate::method::parse_header_attr;
use crate::utils::WithTokens;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, Error, Result, TraitItemMethod};

pub(crate) fn implement_headers(method: &TraitItemMethod) -> Result<TokenStream> {
    let attrs = &method.attrs;
    let headers = attrs
        .iter()
        .filter_map(parse_header_attr)
        .collect::<Vec<_>>();

    let implem = if headers.is_empty() {
        quote! {
            let headers = pretend::HeaderMap::new();
        }
    } else {
        let headers = headers
            .into_iter()
            .map(implement_header_result)
            .collect::<Report<_>>()
            .into_result(|| Error::new_spanned(method, METHOD_FAILURE))?;

        quote! {
            let mut headers = pretend::HeaderMap::new();
            #(#headers)*
        }
    };
    Ok(implem)
}

fn implement_header_result(
    item: WithTokens<Option<(String, String)>, Attribute>,
) -> Result<TokenStream> {
    let value = item.value;
    let tokens = item.tokens;
    let (name, value) = value.ok_or_else(|| Error::new_spanned(tokens, INVALID_HEADER))?;
    Ok(implement_header(name, value))
}

fn implement_header(name: String, value: String) -> TokenStream {
    let name = format(name, "header_name");
    let value = format(value, "header_value");
    quote! {
        #name
        #value
        pretend::internal::build_header(&mut headers, header_name, header_value)?;
    }
}
