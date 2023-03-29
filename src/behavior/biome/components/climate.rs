use serde::{Deserialize, Serialize};
/// Describes temperature, humidity, precipitation, etc.  Biomes without this component will have default values.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Climate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downfall: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub red_spores: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blue_spores: Option<f64>,
    /// UNDOCUMENTED.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ash: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub white_ash: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snow_accumulation: Option<Vec<f64>>,
}
