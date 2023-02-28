use serde::{Deserialize, Serialize};

use crate::general::{identifier::namespace::NamespaceIdentifier, version::Version};

#[derive(Deserialize, Serialize)]
pub struct Fog {
    pub format_version: Version,

    #[serde(rename = "minecraft:fog_settings")]
    pub fog_settings: FogSettings,
}

#[derive(Deserialize, Serialize)]
pub struct FogSettings {
    /// The identifying description of this fog settings.
    pub description: Description,
    /// The distance fog settings for different camera locations.
    pub distance: Distance,
    /// The volumetric fog settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumetric: Option<Volumetric>,
}
#[derive(Deserialize, Serialize)]
/// The identifying description of this fog settings.
pub struct Description {
    pub identifier: NamespaceIdentifier,
}

/// The volumetric fog settings.
#[derive(Deserialize, Serialize)]
pub struct Volumetric {
    /// The density settings for different camera locations.
    pub density: Density,

    /// The coefficient settings for the volumetric fog in different blocks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_coefficients: Option<MediaCoefficients>,
}

/// The coefficient settings for the volumetric fog in different blocks.
#[derive(Deserialize, Serialize)]
pub struct MediaCoefficients {
    /// Fog coefficient values while light passes through air.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub air: Option<VolumeMediaObject>,

    /// Fog coefficient values while light passes through water.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub water: Option<VolumeMediaObject>,

    /// Fog coefficient values while light passes through clouds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud: Option<VolumeMediaObject>,
}

#[derive(Deserialize, Serialize)]
pub struct VolumeMediaObject {
    /// Proportion of light that is absorbed (lost) per block.
    pub absorption: Color,
    /// Proportion of light that is scattered per block.
    pub scattering: Color,
}

/// The density settings for different camera locations.
#[derive(Deserialize, Serialize)]
pub struct Density {
    /// Fog density values as light passes through air blocks.
    pub air: Option<VolumeDensity>,

    /// Fog density values as light passes through water blocks.
    pub water: Option<VolumeDensity>,

    /// Fog density values as light passes through lava blocks.
    pub lava: Option<VolumeDensity>,

    /// Fog density values as light passes through lava blocks while the player has lava resistance.
    pub lava_resistance: Option<VolumeDensity>,
}

#[derive(Deserialize, Serialize)]
pub struct VolumeDensity {
    /// The maximum amount of opaqueness that the ground fog will take on. A value from [0.0, 1.0].
    pub max_density: f32,
    /// The height in blocks that the ground fog will become it's maximum density. Max of 320.
    pub max_density_height: Option<i16>,
    /// The height in blocks that the ground fog will be completely transparent and begin to appear. This value needs to be at least 1 higher than `max_density_height`. Max of 320.
    pub zero_density_height: Option<i16>,
}

/// The distance fog settings for different camera locations.
#[derive(Deserialize, Serialize)]
pub struct Distance {
    /// The fog settings when the camera is in the air.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub air: Option<DefaultFogSettings>,
    /// The fog settings for when the camera is in the air with active weather (rain, snow, etc..).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weather: Option<DefaultFogSettings>,
    /// The fog settings when the camera is in water.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub water: Option<DefaultFogSettings>,
    /// The fog settings when the camera is in lava.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lava: Option<DefaultFogSettings>,
    /// The fog settings when the camera is in lava and the player has the lava resistance effect active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lava_resistance: Option<DefaultFogSettings>,
    /// The fog settings when the camera is inside a Powder Snow block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub powder_snow: Option<DefaultFogSettings>,
}
use serde_json::Number;
#[derive(Deserialize, Serialize)]
pub struct DefaultFogSettings {
    /// The distance from the player that the fog will begin to appear. 'fog_start' must be less than or equal to 'fog_end'.
    pub fog_start: Number,
    /// The distance from the player that the fog will become fully opaque. 'fog_end' must be greater than or equal to 'fog_start'.
    pub fog_end: Number,
    /// The color that the fog will take on.
    pub fog_color: Color,
    /// Determines how distance value is used. Fixed distance is measured in blocks. Dynamic distance is multiplied by the current render distance.
    pub render_distance_type: RenderDistanceType,
    /// Additional fog data which will slowly transition to the distance fog of current biome.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transition_fog: Option<TransitionFog>,
}

#[derive(Deserialize, Serialize)]
pub struct TransitionFog {
    /// Initial fog that will slowly transition into water distance fog of the biome when player goes into water.
    pub init_fog: InitialFog,

    /// The minimum progress of fog transition.
    pub min_percent: f32,

    /// The time takes to reach certain progress('mid_percent') of fog transition.
    pub mid_seconds: u32,

    /// The progress of fog transition after 'mid_seconds' seconds.
    pub mid_percent: f32,

    /// Total amount of time takes to complete fog transition.
    pub max_seconds: u32,
}

/// Initial fog that will slowly transition into water distance fog of the biome when player goes into water.
#[derive(Deserialize, Serialize)]
pub struct InitialFog {
    /// The color that the fog will take on.
    pub fog_color: Color,
    /// The distance from the player that the fog will begin to appear. 'fog_start' must be less than or equal to 'fog_end'.
    pub fog_start: Number,
    /// The distance from the player that the fog will become fully opaque. 'fog_end' must be greater than or equal to 'fog_start'.
    pub fog_end: Number,
    /// Determines how distance value is used. Fixed distance is measured in blocks. Dynamic distance is multiplied by the current render distance.
    pub render_distance_type: RenderDistanceType,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Color {
    Hex(String),
    Array(f32, f32, f32),
}

#[derive(Deserialize, Serialize)]
pub enum RenderDistanceType {
    #[serde(rename = "fixed")]
    Fixed,
    #[serde(rename = "render")]
    Render,
}

#[test]
fn deserialize() {
    let paths = vec![
        "./bedrock-samples/resource_pack/fogs/".to_string(),
        "./Minecraft-bedrock-json-schemas/test/files/correct/data_rp/fogs/".to_string(),
    ];

    crate::utils::test_serde_json_from_files_in_path::<Fog>(paths);
}
