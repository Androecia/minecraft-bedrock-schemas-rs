use crate::{behavior::block::MaterialIdentifier, general::types::FloatRange, molang};
use serde::{Deserialize, Serialize};
/// Specify fine-detail changes to blocks used in terrain generation (based on a noise function).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SurfaceMaterialAdjustments {
    /// All adjustments that match the column's noise values will be applied in the order listed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjustments: Option<Vec<Adjustment>>,
}

///
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Adjustment {
    /// Defines a range of noise values [min, max] for which this adjustment should be applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height_range: Option<Vec<molang::Number>>,

    /// UNDOCUMENTED.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub materials: Option<Materials>,

    /// Defines a range of noise values [min, max] for which this adjustment should be applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub noise_range: Option<FloatRange>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub noise_frequency_scale: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Materials {
    /// Controls the block type used for the surface of this biome when this adjustment is active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_material: Option<MaterialIdentifier>,

    /// Controls the block type used in a layer below the surface of this biome when this adjustment is active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mid_material: Option<MaterialIdentifier>,

    /// Controls the block type used as a floor for bodies of water in this biome when this adjustment is active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sea_floor_material: Option<MaterialIdentifier>,

    /// Controls the block type used deep underground in this biome when this adjustment is active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foundation_material: Option<MaterialIdentifier>,

    /// Controls the block type used in the bodies of water in this biome when this adjustment is active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sea_material: Option<MaterialIdentifier>,
}
