//! Request and response models for the X (Twitter) GraphQL API.
//!
//! The [`request`] and [`response`] modules model individual GraphQL operations, while [`archive`]
//! parses the request/response exchange lines stored in archived captures.
pub mod archive;
pub mod request;
pub mod response;
