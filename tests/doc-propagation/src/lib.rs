//! Test crate

#![forbid(missing_docs)]

use pretend::{pretend, Result};

/// This REST endpoints tests documentation features
#[pretend]
trait TestTrait {
    /// Documentation for a simple method
    #[request(method = "GET", path = "/api/v1/test")]
    async fn test(&self) -> Result<String>;
}
