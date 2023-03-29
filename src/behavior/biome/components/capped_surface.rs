use serde::{Deserialize, Serialize};

use crate::behavior::block::MaterialIdentifier;
/// Generates surface on blocks with non-solid blocks above or below.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CappedSurface {
    /// Materials used for the surface ceiling.
    pub ceiling_materials: Vec<MaterialIdentifier>,
    /// Materials used for the surface floor.
    pub floor_materials: Vec<MaterialIdentifier>,
    /// Material used to replace air blocks below sea level.
    pub sea_material: MaterialIdentifier,
    /// Material used to replace solid blocks that are not surface blocks.
    pub foundation_material: MaterialIdentifier,
    /// Material used to decorate surface near sea level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beach_material: Option<MaterialIdentifier>,
}
