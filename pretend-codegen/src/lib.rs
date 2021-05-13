mod errors;
mod format;
mod method;
mod utils;

use crate::errors::{Report, CODEGEN_FAILURE};
use crate::method::implement_trait_item;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, Error, ItemTrait, Result};

#[proc_macro_attribute]
pub fn request(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn header(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn pretend(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemTrait);
    implement_pretend(item)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

fn implement_pretend(item: ItemTrait) -> Result<TokenStream2> {
    let name = &item.ident;
    let items = &item.items;
    let methods = items
        .iter()
        .map(implement_trait_item)
        .collect::<Report<_>>()
        .into_result(|| Error::new_spanned(name, CODEGEN_FAILURE))?;

    let tokens = quote! {
        #[pretend::client::async_trait]
        trait #name {
            #(#items)*
        }

        #[pretend::client::async_trait]
        impl<C, R> #name for pretend::Pretend<C, R>
            where C: pretend::client::Client + Send + Sync,
                  R: pretend::resolver::ResolveUrl + Send + Sync
        {
            #(#methods)*
        }
    };
    Ok(tokens)
}
