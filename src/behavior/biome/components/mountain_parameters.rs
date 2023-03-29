use serde::{Deserialize, Serialize};

use crate::behavior::block::MaterialIdentifier;
/// Noise parameters used to drive mountain terrain generation in Overworld.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MountainParameters {
    /// UNDOCUMENTED.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peaks_factor: Option<f64>,

    /// Defines surface material for steep slopes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steep_material_adjustment: Option<SteepMaterialAdjustment>,
    /// Controls the density tapering that happens at the top of the world to prevent terrain from reaching too high.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_slide: Option<TopSlide>,
}

/// Defines surface material for steep slopes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SteepMaterialAdjustment {
    /// Block type use as steep material.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub material: Option<MaterialIdentifier>,
    /// Enable for north facing slopes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub north_slopes: Option<bool>,
    /// Enable for south facing slopes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub south_slopes: Option<bool>,
    /// Enable for west facing slopes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub west_slopes: Option<bool>,
    /// Enable for east facing slopes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub east_slopes: Option<bool>,
}

/// Controls the density tapering that happens at the top of the world to prevent terrain from reaching too high.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopSlide {
    /// If false, top slide will be disabled. If true, other parameters will be taken into account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
