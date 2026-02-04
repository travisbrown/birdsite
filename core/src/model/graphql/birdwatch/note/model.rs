#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ModelVersion {
    V1_0,
    V1_1,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Model {
    #[serde(rename = "CoreModel (v1.1)")]
    CoreModel,
    #[serde(rename = "ExpansionModel (v1.1)")]
    ExpansionModel,
    #[serde(rename = "ExpansionPlusModel (v1.1)")]
    ExpansionPlusModel,
    #[serde(rename = "GroupModel01 (v1.1)")]
    GroupModel01,
    #[serde(rename = "GroupModel02 (v1.1)")]
    GroupModel02,
    #[serde(rename = "GroupModel03 (v1.1)")]
    GroupModel03,
    #[serde(rename = "GroupModel04 (v1.1)")]
    GroupModel04,
    #[serde(rename = "GroupModel05 (v1.1)")]
    GroupModel05,
    #[serde(rename = "GroupModel06 (v1.1)")]
    GroupModel06,
    #[serde(rename = "GroupModel07 (v1.1)")]
    GroupModel07,
    #[serde(rename = "GroupModel08 (v1.1)")]
    GroupModel08,
    #[serde(rename = "GroupModel09 (v1.1)")]
    GroupModel09,
    #[serde(rename = "GroupModel10 (v1.1)")]
    GroupModel10,
    #[serde(rename = "GroupModel11 (v1.1)")]
    GroupModel11,
    #[serde(rename = "GroupModel12 (v1.1)")]
    GroupModel12,
    #[serde(rename = "GroupModel13 (v1.1)")]
    GroupModel13,
    #[serde(rename = "GroupModel14 (v1.1)")]
    GroupModel14,
    #[serde(rename = "GroupModel33 (v1.1)")]
    GroupModel33,
    #[serde(rename = "GroupModel33NMR (v1.1)")]
    GroupModel33Nmr,
    #[serde(rename = "InsufficientExplanation (v1.0)")]
    InsufficientExplanation,
    #[serde(rename = "MultiGroupModel01 (v1.0)")]
    MultiGroupModel01,
    #[serde(rename = "NmrDueToMinStableCrhTime (v1.0)")]
    NmrDueToMinStableCrhTime,
    #[serde(rename = "ScoringDriftGuard (v1.0)")]
    ScoringDriftGuard,
    #[serde(rename = "TopicModel01 (v1.0)")]
    TopicModel01,
    #[serde(rename = "TopicModel02 (v1.0)")]
    TopicModel02,
    #[serde(rename = "TopicModel03 (v1.0)")]
    TopicModel03,
    #[serde(rename = "TopicModel04 (v1.0)")]
    TopicModel04,
    #[serde(rename = "CoreWithTopicsModel (v1.1)")]
    CoreWithTopicsModel,
    #[serde(rename = "GaussianModel (v1.0)")]
    GaussianModel,
    #[serde(rename = "PopulationSampledIntercept (v1.0)")]
    PopulationSampledIntercept,
}

impl Model {
    #[must_use]
    pub const fn version(&self) -> ModelVersion {
        match self {
            Self::InsufficientExplanation
            | Self::MultiGroupModel01
            | Self::NmrDueToMinStableCrhTime
            | Self::ScoringDriftGuard
            | Self::TopicModel01
            | Self::TopicModel02
            | Self::TopicModel03
            | Self::TopicModel04
            | Self::GaussianModel
            | Self::PopulationSampledIntercept => ModelVersion::V1_0,
            _ => ModelVersion::V1_1,
        }
    }
}
