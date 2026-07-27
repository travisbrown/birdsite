#![warn(clippy::all, clippy::pedantic, clippy::nursery, rust_2018_idioms)]
#![allow(clippy::missing_errors_doc)]
#![forbid(unsafe_code)]
//! Data models for working with archived X (Twitter) data.
pub mod model;
#[cfg(test)]
mod test_support;
