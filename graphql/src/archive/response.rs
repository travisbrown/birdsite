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
