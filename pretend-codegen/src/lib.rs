mod errors;
mod format;
mod method;
mod utils;

use crate::errors::{Report, CODEGEN_FAILURE, INVALID_CLIENT_KIND};
use crate::method::implement_trait_item;
use crate::utils::Single;
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, Error, ItemTrait, Meta, NestedMeta, Result};

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
    let attr = parse_macro_input!(attr as AttributeArgs);
    let item = parse_macro_input!(item as ItemTrait);
    implement_pretend(attr, item)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

fn implement_pretend(attr: AttributeArgs, item: ItemTrait) -> Result<TokenStream2> {
    let kind = parse_client_kind(attr)?;

    let name = &item.ident;
    let items = &item.items;
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
        trait #name {
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
    Futures,
    FuturesNoSend,
    Blocking,
}

fn parse_client_kind(args: AttributeArgs) -> Result<ClientKind> {
    let single = args
        .into_iter()
        .map(parse_client_kind_attr)
        .collect::<Single<_>>();
    match single {
        Single::None => Ok(ClientKind::Futures),
        Single::Single(single) => {
            single.ok_or_else(|| Error::new(Span::call_site(), INVALID_CLIENT_KIND))
        }
        Single::TooMany(_) => Err(Error::new(Span::call_site(), INVALID_CLIENT_KIND)),
    }
}

fn parse_client_kind_attr(nested: NestedMeta) -> Option<ClientKind> {
    match nested {
        NestedMeta::Meta(Meta::Path(path)) => {
            let ident = path.get_ident()?;
            if ident == "blocking" {
                Some(ClientKind::Blocking)
            } else if ident == "non_send" {
                Some(ClientKind::FuturesNoSend)
            } else {
                None
            }
        }
        _ => None,
    }
}

fn async_trait_attr(kind: &ClientKind) -> TokenStream2 {
    match kind {
        ClientKind::Futures => quote! {
            #[pretend::client::async_trait]
        },
        ClientKind::FuturesNoSend => quote! {
            #[pretend::client::async_trait(?Send)]
        },
        ClientKind::Blocking => TokenStream2::new(),
    }
}

fn client_implem(kind: &ClientKind) -> TokenStream2 {
    match kind {
        ClientKind::Futures => quote! {
            pretend::client::Client
        },
        ClientKind::FuturesNoSend => quote! {
            pretend::client::NonSendClient
        },
        ClientKind::Blocking => quote! {
            pretend::client::BlockingClient
        },
    }
}

fn send_sync_traits_impl(kind: &ClientKind) -> TokenStream2 {
    match kind {
        ClientKind::Futures => quote! {
            + Send + Sync
        },
        ClientKind::FuturesNoSend => TokenStream2::new(),
        ClientKind::Blocking => TokenStream2::new(),
    }
}
