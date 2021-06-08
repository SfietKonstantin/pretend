use crate::utils::{parse_name_value_2_attr, WithTokens};
use syn::Attribute;

pub(crate) fn parse_request_attr(
    attr: &Attribute,
) -> Option<WithTokens<Option<(String, String)>, Attribute>> {
    parse_name_value_2_attr(attr, "request", "method", "path")
}

pub(crate) fn parse_header_attr(
    attr: &Attribute,
) -> Option<WithTokens<Option<(String, String)>, Attribute>> {
    parse_name_value_2_attr(attr, "header", "name", "value")
}
