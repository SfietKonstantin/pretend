use crate::format::format;
use crate::utils::parse_name_value_attr;
use proc_macro2::TokenStream;
use quote::quote;
use std::iter::FromIterator;
use syn::{Attribute, TraitItemMethod};

pub(crate) fn implement_headers(method: &TraitItemMethod) -> TokenStream {
    let headers = method
        .attrs
        .iter()
        .filter_map(parse_header_attr)
        .collect::<Vec<_>>();

    if headers.is_empty() {
        quote! {
            let headers = pretend::HeaderMap::new();
        }
    } else {
        let headers = headers
            .into_iter()
            .map(|(name, value)| implement_header(name, value))
            .collect::<Vec<_>>();

        quote! {
            let mut headers = pretend::HeaderMap::new();
            #(#headers)*
        }
    }
}

fn parse_header_attr(attr: &Attribute) -> Option<(String, String)> {
    let header = parse_name_value_attr("header", attr)?
        .into_iter()
        .collect::<Header>();
    let (name, value) = header.into_header()?;
    Some((name, value))
}

struct Header {
    name: Option<String>,
    value: Option<String>,
}

impl Header {
    fn into_header(self) -> Option<(String, String)> {
        Some((self.name?, self.value?))
    }
}

impl FromIterator<(String, String)> for Header {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (String, String)>,
    {
        let mut header_name = None;
        let mut header_value = None;

        for (key, value) in iter {
            match key.as_str() {
                "name" => header_name = Some(value),
                "value" => header_value = Some(value),
                _ => {}
            }
        }

        Header {
            name: header_name,
            value: header_value,
        }
    }
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
