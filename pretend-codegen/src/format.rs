use lazy_static::lazy_static;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use regex::Regex;

pub(crate) fn format(string: String, param: &str) -> TokenStream {
    let param = Ident::new(param, Span::call_site());
    let params = find_params(&string);
    if params.is_empty() {
        quote! {
            let #param = #string;
        }
    } else {
        let params = params
            .into_iter()
            .map(|param| Ident::new(param, Span::call_site()))
            .collect::<Vec<_>>();

        quote! {
            let #param = format!(#string, #(#params=#params,)*);
            let #param = #param.as_str();
        }
    }
}

lazy_static! {
    static ref PARAM_RE: Regex = Regex::new(r"\{([^}]+)\}").unwrap();
}

fn find_params(path: &str) -> Vec<&str> {
    PARAM_RE
        .captures_iter(path)
        .filter_map(|cap| cap.get(1))
        .map(|m| m.as_str())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_params() {
        let path = "/{user}/{id}";
        let params = find_params(path);
        assert_eq!(params, vec!["user", "id"]);
    }

    #[test]
    fn test_find_no_params() {
        let path = "/{}";
        let params = find_params(path);
        assert_eq!(params, Vec::<&str>::new());
    }
}
