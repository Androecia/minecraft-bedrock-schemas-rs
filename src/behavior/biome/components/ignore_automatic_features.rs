use serde::{Deserialize, Serialize};
/// No features will be automatically attached to this Biome, only features specified in the minecraft:forced_features component will appear.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IgnoreAutomaticFeatures {}
