use serde::{Deserialize, Serialize};

use crate::behavior::block::MaterialIdentifier;
/// Similar to overworld_surface. Adds icebergs.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FrozenOceanSurface {
    /// Controls the block type used for the surface of this biome.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_material: Option<MaterialIdentifier>,
    /// Controls the block type used in a layer below the surface of this biome.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mid_material: Option<MaterialIdentifier>,
    /// Controls the block type used as a floor for bodies of water in this biome.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sea_floor_material: Option<MaterialIdentifier>,
    /// Controls the block type used deep underground in this biome.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foundation_material: Option<MaterialIdentifier>,
    /// Controls the block type used for the bodies of water in this biome.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sea_material: Option<MaterialIdentifier>,
    /// Controls how deep below the world water level the floor should occur.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sea_floor_depth: Option<i32>,
}
