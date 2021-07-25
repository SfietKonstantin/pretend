mod attr;
mod errors;
mod format;
mod method;
mod utils;

use crate::attr::PretendAttr;
use crate::errors::{
    ErrorsExt, Report, CODEGEN_FAILURE, INCONSISTENT_ASYNC, INCONSISTENT_ASYNC_ASYNC_HINT,
    INCONSISTENT_ASYNC_NON_ASYNC_HINT, NO_METHOD, UNSUPPORTED_ATTR_SYNC,
};
use crate::method::implement_trait_item;
use crate::utils::WithTokens;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{parse_macro_input, Error, ItemTrait, Result, Signature, TraitItem};

#[proc_macro_attribute]
pub fn request(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn header(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn pretend(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = parse_macro_input!(attr as PretendAttr);
    let item = parse_macro_input!(item as ItemTrait);
    implement_pretend(attr, item)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

fn implement_pretend(attr: PretendAttr, item: ItemTrait) -> Result<TokenStream2> {
    let name = &item.ident;
    let vis = &item.vis;
    let items = &item.items;
    let kind = parse_client_kind(name, attr, items)?;
    let methods = items
        .iter()
        .map(|item| implement_trait_item(item, &kind))
        .collect::<Report<_>>()
        .into_result(|| Error::new(Span::call_site(), CODEGEN_FAILURE))?;

    let attr = async_trait_attr(&kind);
    let client = client_implem(&kind);
    let send_sync = send_sync_traits_impl(&kind);
    let tokens = quote! {
        #attr
        #vis trait #name {
            #(#items)*
        }

        #attr
        impl<C, R> #name for pretend::Pretend<C, R>
            where C: #client #send_sync,
                  R: pretend::resolver::ResolveUrl #send_sync
        {
            #(#methods)*
        }
    };
    Ok(tokens)
}

enum ClientKind {
    Async,
    AsyncLocal,
    Blocking,
}

fn parse_client_kind(name: &Ident, attr: PretendAttr, items: &[TraitItem]) -> Result<ClientKind> {
    let asyncs = items.iter().filter_map(is_method_async).collect::<Vec<_>>();
    let is_async = asyncs.iter().all(|item| item.value);
    let is_not_async = asyncs.iter().all(|item| !item.value);

    match (is_async, is_not_async) {
        (true, false) => {
            if attr.local {
                Ok(ClientKind::AsyncLocal)
            } else {
                Ok(ClientKind::Async)
            }
        }
        (false, true) => {
            if attr.local {
                Err(Error::new(Span::call_site(), UNSUPPORTED_ATTR_SYNC))
            } else {
                Ok(ClientKind::Blocking)
            }
        }
        _ => {
            if asyncs.is_empty() {
                Err(Error::new_spanned(name, NO_METHOD))
            } else {
                let async_hints = asyncs
                    .iter()
                    .filter(|item| item.value)
                    .map(|item| Error::new_spanned(item.tokens, INCONSISTENT_ASYNC_ASYNC_HINT));

                let non_async_hints = asyncs
                    .iter()
                    .filter(|item| !item.value)
                    .map(|item| Error::new_spanned(item.tokens, INCONSISTENT_ASYNC_NON_ASYNC_HINT));

                let errors = async_hints.chain(non_async_hints).collect::<Vec<_>>();
                errors.into_result(|| Error::new_spanned(name, INCONSISTENT_ASYNC))
            }
        }
    }
}

fn is_method_async(item: &TraitItem) -> Option<WithTokens<bool, Signature>> {
    match item {
        TraitItem::Method(method) => {
            let is_async = method.sig.asyncness.is_some();
            Some(WithTokens::new(is_async, &method.sig))
        }
        _ => None,
    }
}

fn async_trait_attr(kind: &ClientKind) -> TokenStream2 {
    match kind {
        ClientKind::Async => quote! {
            #[pretend::client::async_trait]
        },
        ClientKind::AsyncLocal => quote! {
            #[pretend::client::async_trait(?Send)]
        },
        ClientKind::Blocking => TokenStream2::new(),
    }
}

fn client_implem(kind: &ClientKind) -> TokenStream2 {
    match kind {
        ClientKind::Async => quote! {
            pretend::client::Client
        },
        ClientKind::AsyncLocal => quote! {
            pretend::local::client::Client
        },
        ClientKind::Blocking => quote! {
            pretend::client::BlockingClient
        },
    }
}

fn send_sync_traits_impl(kind: &ClientKind) -> TokenStream2 {
    match kind {
        ClientKind::Async => quote! {
            + Send + Sync
        },
        ClientKind::AsyncLocal => TokenStream2::new(),
        ClientKind::Blocking => TokenStream2::new(),
    }
}
