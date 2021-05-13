use super::WithTokens;
use syn::{Attribute, Lit, Meta, MetaList, MetaNameValue, NestedMeta};

pub(crate) fn parse_name_value_2_attr<'a>(
    attr: &'a Attribute,
    name: &'static str,
    key_name: &'static str,
    value_name: &'static str,
) -> Option<WithTokens<'a, Option<(String, String)>, Attribute>> {
    let name_values = parse_name_value_attr(attr, name)?;

    let mut key_result = None;
    let mut value_result = None;
    let mut valid = true;

    for (name, value) in name_values {
        if name == key_name {
            key_result = Some(value);
        } else if name == value_name {
            value_result = Some(value);
        } else {
            valid = false;
        }
    }

    if let (Some(key), Some(value), true) = (key_result, value_result, valid) {
        Some(WithTokens::new(Some((key, value)), attr))
    } else {
        Some(WithTokens::new(None, attr))
    }
}

// Returns a list of name-value
fn parse_name_value_attr(attr: &Attribute, name: &str) -> Option<Vec<(String, String)>> {
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
