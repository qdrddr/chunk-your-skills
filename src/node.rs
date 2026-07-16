#[path = "pageindex_node.rs"]
mod pageindex_node;

#[path = "cache_node.rs"]
mod cache_node;

pub use pageindex_node::*;

use napi_derive::napi;

#[napi]
#[must_use]
pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
