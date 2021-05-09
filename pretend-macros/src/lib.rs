mod errors;
mod format;
mod method;
mod utils;

use crate::errors::IResult;
use crate::method::implement_trait_item;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, ItemTrait};

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
    let result = implement_pretend(item);
    match result {
        Ok(result) => result.into(),
        Err(err) => panic!("{}", err),
    }
}

fn implement_pretend(item: ItemTrait) -> IResult<TokenStream2> {
    let name = item.ident;
    let items = item.items;
    let methods = items
        .iter()
        .map(implement_trait_item)
        .collect::<IResult<Vec<_>>>()?;

    let tokens = quote! {
        #[pretend::async_trait]
        trait #name {
            #(#items)*
        }

        #[pretend::async_trait]
        impl<C, R> #name for pretend::Pretend<C, R>
            where C: pretend::client::Client + Send + Sync,
                  R: pretend::ResolveUrl + Send + Sync
        {
            #(#methods)*
        }
    };
//    panic!("{}", tokens);
    Ok(tokens)
}
