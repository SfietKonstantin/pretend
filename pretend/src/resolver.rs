//! URL resolver
//!
//! `pretend` supports URL resolvers. They are used to
//! create a full URL from the path specified in the
//! `request` attribute.
//!
//! By default [`UrlResolver`] is used to append the path
//! to a base URL. This resolver is used in `[Pretend::with_url]`.
//!
//! You can implement your own resolvers to suit your needs. For
//! example, you can delegate URL resolution to a load balancer.
//! In this case, implement [`ResolveUrl`].

pub use url::{ParseError, Url};

/// Describe an URL resolver
///
/// See module level documentation for more information.
pub trait ResolveUrl {
    /// Resolve an URL from a path
    fn resolve_url(&self, path: &str) -> Result<Url, ParseError>;
}

/// Default URL resolver
///
/// This resolver appends the path to a base URL.
#[derive(Clone, Debug)]
pub struct UrlResolver {
    base: Url,
}

impl UrlResolver {
    /// Constructor
    pub fn new(base: Url) -> Self {
        UrlResolver { base }
    }
}

impl ResolveUrl for UrlResolver {
    fn resolve_url(&self, path: &str) -> Result<Url, ParseError> {
        self.base.join(path)
    }
}

/// Invalid URL resolver
///
/// This resolver is used when calling `[Pretend::for_client]`. It
/// is will raise an error when resolving any input.
///
/// `[Pretend::with_url]` or `[Pretend::with_url_resolver]` should be used to
/// set a valid URL resolver.
#[derive(Clone, Copy, Debug)]
pub struct InvalidUrlResolver;

impl ResolveUrl for InvalidUrlResolver {
    fn resolve_url(&self, _: &str) -> Result<Url, ParseError> {
        Err(ParseError::EmptyHost)
    }
}
