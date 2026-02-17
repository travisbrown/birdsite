use bounded_static::{IntoBoundedStatic, ToBoundedStatic};

pub mod io;
pub mod parse;
pub mod request;
pub mod response;

#[derive(Clone, Debug)]
pub struct Exchange<'a, V, D> {
    pub request: request::Request<'a, V>,
    pub data: Option<D>,
    pub errors: Vec<crate::response::error::Error>,
}

impl<'a, V: IntoBoundedStatic, D: IntoBoundedStatic> IntoBoundedStatic for Exchange<'a, V, D> {
    type Static = Exchange<'static, V::Static, D::Static>;

    fn into_static(self) -> Self::Static {
        Self::Static {
            request: self.request.into_static(),
            data: self.data.into_static(),
            errors: self.errors,
        }
    }
}

impl<'a, V: ToBoundedStatic, D: ToBoundedStatic> ToBoundedStatic for Exchange<'a, V, D> {
    type Static = Exchange<'static, V::Static, D::Static>;

    fn to_static(&self) -> Self::Static {
        Self::Static {
            request: self.request.to_static(),
            data: self.data.to_static(),
            errors: self.errors.clone(),
        }
    }
}
