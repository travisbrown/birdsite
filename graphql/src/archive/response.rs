#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("JSON decoding")]
    Json(#[from] serde_json::Error),
    #[error("Result length does not match request")]
    InvalidResultLength { expected: usize, returned: usize },
}

pub trait ParseWithVariables<'a, V> {
    fn parse(input: &'a str, variables: &V) -> Result<Self, Error>
    where
        Self: Sized + 'a;
}

pub struct JsonResponse(pub serde_json::Value);

impl<'a, V> ParseWithVariables<'a, V> for JsonResponse {
    fn parse(input: &'a str, _variables: &V) -> Result<Self, Error>
    where
        Self: Sized + 'a,
    {
        Ok(Self(serde_json::from_str(input).map_err(Error::from)?))
    }
}

impl bounded_static::IntoBoundedStatic for JsonResponse {
    type Static = Self;

    fn into_static(self) -> Self::Static {
        self
    }
}
