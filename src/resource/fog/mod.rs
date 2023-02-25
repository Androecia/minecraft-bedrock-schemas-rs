use std::fmt::{self, LowerHex};

use serde::{
    de::{SeqAccess, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};

use crate::general::{identifier::Identifier, version::Version};
/*
{
    "$schema": "http://json-schema.org/draft-07/schema",
    "$id": "blockception.minecraft.resource.fog",
    "examples": [],
    "type": "object",
    "additionalProperties": false,
    "title": "Fog",
    "description": "UNDOCUMENTED.",
    "$comment": "UNDOCUMENTED",
    "definitions": {
        "colorHexOrArray": {
            "examples": ["#056bd1"],
            "oneOf": [
                {
                    "type": "array",
                    "items": [
                        { "type": "number", "minimum": 0, "maximum": 1, "title": "Red" },
                        { "type": "number", "minimum": 0, "maximum": 1, "title": "Green" },
                        { "type": "number", "minimum": 0, "maximum": 1, "title": "Blue" }
                    ]
                },
                { "type": "string", "format": "color-hex", "pattern": "^\\#[0-9a-fA-F]{6}$" }
            ]
        },

        "volumeDensityObject": {
            "type": "object",
            "additionalProperties": false,
            "required": ["max_density"],
            "examples": [{ "max_density": 0.25 }, { "max_density": 0.25, "max_density_height": 128, "zero_density_height": 20, "uniform": true }],
            "properties": {
                "max_density": {
                    "title": "Maximum Density",
                    "description": "The maximum amount of opaqueness that the ground fog will take on. A value from [0.0, 1.0].",
                    "minimum": 0,
                    "maximum": 1,
                    "type": "number"
                },
                "max_density_height": {
                    "title": "Maximum Density Height",
                    "description": "The height in blocks that the ground fog will become it's maximum density.",
                    "minimum": 0,
                    "maximum": 128,
                    "type": "number"
                },
                "zero_density_height": {
                    "title": "Zero Density Height",
                    "description": "The height in blocks that the ground fog will be completely transparent and begin to appear. This value needs to be at least 1 higher than `max_density_height`.",
                    "minimum": 0,
                    "maximum": 128,
                    "type": "number"
                },
                "uniform": { "title": "Uniform", "description": "When set to true, the density will be uniform across all heights.", "type": "boolean" }
            }
        },
        "volumeMediaObject": {
            "type": "object",
            "additionalProperties": false,
            "properties": {
                "absorption": {
                    "title": "Absorption",
                    "description": "Proportion of light that is absorbed (lost) per block.",
                    "$ref": "#/definitions/colorHexOrArray"
                },
                "scattering": {
                    "title": "Scattering",
                    "description": "Proportion of light that is scattered per block.",
                    "$ref": "#/definitions/colorHexOrArray"
                }
            }
        }
    },
    "properties": {
        "format_version": { "$ref": "../../general/format_version.json" },
        "minecraft:fog_settings": {
            "title": "Fog Settings",
            "description": "The definition of a single fog.",
            "type": "object",
            "additionalProperties": false,
            "properties": {
                "description": {
                    "title": "Description",
                    "description": "The identifying description of this fog settings.",
                    "type": "object",
                    "additionalProperties": false,
                    "properties": {
                        "identifier": {
                            "type": "string",
                            "title": "Identifier",
                            "description": "The identifier for these fog settings. The identifier must include a namespace.",
                            "$ref": "../../general/fog/identifier.json"
                        }
                    }
                },
                "distance": {
                    "title": "Distance",
                    "description": "The distance fog settings for different camera locations.",
                    "type": "object",
                    "additionalProperties": false,
                    "properties": {
                        "air": { "title": "Air", "description": "The fog settings when the camera is in the air.", "$ref": "#/definitions/defaultFogSettings" },
                        "weather": {
                            "title": "Weather",
                            "description": " The fog settings for when the camera is in the air with active weather (rain, snow, etc..).",
                            "$ref": "#/definitions/defaultFogSettings"
                        },
                        "water": { "title": "Water", "description": "The fog settings when the camera is in water.", "$ref": "#/definitions/defaultFogSettings" },
                        "lava": { "title": "Lava", "description": "The fog settings when the camera is in lava.", "$ref": "#/definitions/defaultFogSettings" },
                        "lava_resistance": {
                            "title": "Lava Resistance",
                            "description": "The fog settings when the camera is in lava and the player has the lava resistance effect active.",
                            "$ref": "#/definitions/defaultFogSettings"
                        },
                        "powder_snow": {
                            "title": "Powder Snow",
                            "description": "The fog settings when the camera is inside a Powder Snow block.",
                            "$ref": "#/definitions/defaultFogSettings"
                        }
                    }
                },
                "volumetric": {
                    "title": "Volumetric",
                    "description": "The volumetric fog settings.",
                    "type": "object",
                    "additionalProperties": false,
                    "properties": {
                        "density": {
                            "title": "Density",
                            "description": "The density settings for different camera locations.",
                            "type": "object",
                            "additionalProperties": false,
                            "properties": {
                                "air": {
                                    "title": "Air",
                                    "description": "Fog density values as light passes through air blocks.",
                                    "$ref": "#/definitions/volumeDensityObject"
                                },
                                "water": {
                                    "title": "Water",
                                    "description": "Fog density values as light passes through water blocks.",
                                    "$ref": "#/definitions/volumeDensityObject"
                                },
                                "lava": {
                                    "title": "Lava",
                                    "description": "Fog density values as light passes through lava blocks.",
                                    "$ref": "#/definitions/volumeDensityObject"
                                },
                                "lava_resistance": {
                                    "title": "Lava Resistance",
                                    "description": "Fog density values as light passes through lava blocks while the player has lava resistance.",
                                    "$ref": "#/definitions/volumeDensityObject"
                                }
                            }
                        },
                        "media_coefficients": {
                            "title": "Media Coefficients",
                            "description": "The coefficient settings for the volumetric fog in different blocks.",
                            "additionalProperties": false,
                            "type": "object",
                            "properties": {
                                "air": {
                                    "title": "Air",
                                    "description": "Fog coefficient values while light passes through air.",
                                    "$ref": "#/definitions/volumeMediaObject"
                                },
                                "water": {
                                    "title": "Water",
                                    "description": "Fog coefficient values while light passes through water.",
                                    "$ref": "#/definitions/volumeMediaObject"
                                },
                                "cloud": {
                                    "title": "Cloud",
                                    "description": "Fog coefficient values while light passes through clouds.",
                                    "$ref": "#/definitions/volumeMediaObject"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
 */
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
    pub identifier: Identifier,
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
        "../Minecraft-bedrock-json-schemas/test/files/correct/data_rp/fogs/".to_string(),
    ];

    crate::test_utils::test_serde_json_from_files_in_path::<Fog>(paths);
}
