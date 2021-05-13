use crate::errors::INVALID_ATTR;
use proc_macro2::Span;
use syn::parse::{Error, Parse, ParseStream, Result};
use syn::Token;

pub(crate) struct PretendAttr {
    pub local: bool,
}

mod kw {
    syn::custom_keyword!(Send);
}

impl Parse for PretendAttr {
    fn parse(input: ParseStream) -> Result<Self> {
        match try_parse(input) {
            Ok(args) if input.is_empty() => Ok(args),
            _ => Err(Error::new(Span::call_site(), INVALID_ATTR)),
        }
    }
}

fn try_parse(input: ParseStream) -> Result<PretendAttr> {
    if input.peek(Token![?]) {
        input.parse::<Token![?]>()?;
        input.parse::<kw::Send>()?;
        Ok(PretendAttr { local: true })
    } else {
        Ok(PretendAttr { local: false })
    }
}
