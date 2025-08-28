#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("JSON decoding")]
    Json(#[from] serde_json::Error),
    #[error("Result length does not match request")]
    InvalidResultLength { expected: usize, returned: usize },
}

pub trait ParseWithVariables<V> {
    fn parse(input: &str, variables: &V) -> Result<Self, Error>
    where
        Self: Sized;
}
