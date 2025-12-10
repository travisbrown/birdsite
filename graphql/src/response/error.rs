#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum Error {
    Structured {
        code: usize,
        message: String,
        name: Option<String>,
        source: Option<Source>,
        kind: Option<Kind>,
        locations: Option<Vec<Location>>,
        path: Option<Vec<PathEntry>>,
    },
    StructuredWithExtensions {
        extensions: Extensions,
        message: String,
        path: Vec<PathEntry>,
    },
    Message(String),
}

#[derive(
    Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum PathEntry {
    Field(String),
    Index(usize),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Source {
    Client,
    Server,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Kind {
    NonFatal,
    Operational,
    Permissions,
    ServiceLevel,
    Unknown,
    Validation,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Extensions {
    pub code: Option<usize>,
    pub kind: Kind,
    pub name: String,
    pub source: Source,
}
