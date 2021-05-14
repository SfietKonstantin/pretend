mod attr;
mod single;
mod withtokens;

pub(crate) use attr::parse_name_value_2_attr;
pub(crate) use single::Single;
pub(crate) use withtokens::WithTokens;

use proc_macro2::Ident;
use syn::{FnArg, Pat};

pub(crate) fn parse_param_name(input: &FnArg) -> Option<&Ident> {
    match input {
        FnArg::Typed(param) => match &*param.pat {
            Pat::Ident(pat) => Some(&pat.ident),
            _ => None,
        },
        _ => None,
    }
}
