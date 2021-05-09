use proc_macro2::Ident;
use syn::{Attribute, FnArg, Lit, Meta, MetaList, MetaNameValue, NestedMeta, Pat};

pub(crate) fn parse_param_name(input: &FnArg) -> Option<&Ident> {
    match input {
        FnArg::Typed(param) => match &*param.pat {
            Pat::Ident(pat) => Some(&pat.ident),
            _ => None,
        },
        _ => None,
    }
}

pub(crate) fn parse_name_value_attr(name: &str, attr: &Attribute) -> Option<Vec<(String, String)>> {
    let list = get_meta_list(attr)?;
    let path = list.path.get_ident()?;
    if path == name {
        let nested = list.nested;
        let attr = nested.iter().filter_map(parse_nested_meta).collect();
        Some(attr)
    } else {
        None
    }
}

fn get_meta_list(attr: &Attribute) -> Option<MetaList> {
    let meta = attr.parse_meta().ok()?;
    match meta {
        Meta::List(list) => Some(list),
        _ => None,
    }
}

fn parse_nested_meta(meta: &NestedMeta) -> Option<(String, String)> {
    match meta {
        NestedMeta::Meta(Meta::NameValue(name_value)) => parse_meta_name_value(name_value),
        _ => None,
    }
}

fn parse_meta_name_value(name_value: &MetaNameValue) -> Option<(String, String)> {
    let path = name_value.path.get_ident();
    match (path, &name_value.lit) {
        (Some(path), Lit::Str(value)) => Some((path.to_string(), value.value())),
        _ => None,
    }
}
