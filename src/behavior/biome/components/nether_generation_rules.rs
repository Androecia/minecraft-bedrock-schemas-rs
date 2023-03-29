use serde::{Deserialize, Serialize};
/// Controls how this biome is instantiated (and then potentially modified) during world generation of the nether.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetherGenerationRules {
    /// Temperature with which this biome should selected, relative to other biomes.
    pub target_temperature: f64,

    /// Humidity with which this biome should selected, relative to other biomes.
    pub target_humidity: f64,

    /// Altitude with which this biome should selected, relative to other biomes.
    pub target_altitude: f64,

    /// Weirdness with which this biome should selected, relative to other biomes.
    pub target_weirdness: f64,

    /// Weight with which this biome should selected, relative to other biomes.
    pub weight: f64,
}
