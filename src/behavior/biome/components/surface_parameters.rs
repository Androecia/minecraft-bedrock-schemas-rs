use serde::{Deserialize, Serialize};

use crate::behavior::block::MaterialIdentifier;

/// Control the blocks used for the default Minecraft Overworld terrain generation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SurfaceParameters {
    /// Controls the block type used for the surface of this biome.
    pub top_material: Option<MaterialIdentifier>,

    /// Controls the block type used in a layer below the surface of this biome.
    pub mid_material: Option<MaterialIdentifier>,

    /// Controls the block type used as a floor for bodies of water in this biome.
    pub sea_floor_material: Option<MaterialIdentifier>,

    /// Controls the block type used deep underground in this biome.
    pub foundation_material: Option<MaterialIdentifier>,

    /// Controls the block type used for the bodies of water in this biome.
    pub sea_material: Option<MaterialIdentifier>,

    /// Controls how deep below the world water level the floor should occur.
    pub sea_floor_depth: Option<i32>,
}
