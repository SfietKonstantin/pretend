use crate::utils::parse_param_name;
use proc_macro2::TokenStream;
use quote::quote;
use syn::TraitItemMethod;

pub(crate) fn implement_query(method: &TraitItemMethod) -> TokenStream {
    if has_query(method) {
        quote! {
            let url = pretend::internal::build_query(url, &query)?;
        }
    } else {
        TokenStream::new()
    }
}

fn has_query(method: &TraitItemMethod) -> bool {
    let inputs = &method.sig.inputs;
    inputs
        .iter()
        .filter_map(parse_param_name)
        .any(|param| param == "query")
}
