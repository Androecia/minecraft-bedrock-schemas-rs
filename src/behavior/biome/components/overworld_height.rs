use serde::{Deserialize, Serialize};

use crate::general::types::FloatRange;
/// Noise parameters used to drive terrain height in the Overworld.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OverworldHeight {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub noise_params: Option<FloatRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub noise_type: Option<NoiseType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NoiseType {
    #[serde(rename = "stone_beach")]
    StoneBeach,
    #[serde(rename = "deep_ocean")]
    DeepOcean,
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "default_mutated")]
    DefaultMutated,
    #[serde(rename = "lowlands")]
    Lowlands,
    #[serde(rename = "river")]
    River,
    #[serde(rename = "ocean")]
    Ocean,
    #[serde(rename = "taiga")]
    Taiga,
    #[serde(rename = "mountains")]
    Mountains,
    #[serde(rename = "highlands")]
    Highlands,
    #[serde(rename = "mushroom")]
    Mushroom,
    #[serde(rename = "less_extreme")]
    LessExtreme,
    #[serde(rename = "extreme")]
    Extreme,
    #[serde(rename = "beach")]
    Beach,
    #[serde(rename = "swamp")]
    Swamp,
}
