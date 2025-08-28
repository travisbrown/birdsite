pub mod parse;
pub mod request;
pub mod response;

#[derive(Clone, Debug)]
pub struct Exchange<'a, V, R> {
    pub request: request::Request<'a, V>,
    pub data: Option<R>,
    pub errors: Vec<crate::response::error::Error>,
}
