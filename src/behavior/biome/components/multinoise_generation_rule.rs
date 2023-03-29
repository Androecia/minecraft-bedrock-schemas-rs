use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MultinoiseGenerationRules {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_altitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_humidity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_weirdness: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<f64>,
}
