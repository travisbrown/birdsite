#![warn(clippy::all, clippy::pedantic, clippy::nursery, rust_2018_idioms)]
#![allow(clippy::missing_errors_doc)]
#![forbid(unsafe_code)]
//! Request and response models for the X (Twitter) GraphQL API.
//!
//! The [`request`] and [`response`] modules model individual GraphQL operations, while [`archive`]
//! parses the request/response exchange lines stored in archived captures.
pub mod archive;
pub mod request;
pub mod response;
