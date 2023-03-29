use crate::{
    general::{identifier::animation::AnimationIdentifier, version::Version},
    molang::{self, VariableDefinition},
    types::{ArrayOrSingle, Timeline},
};
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::{collections::HashMap, fmt};

use super::animation_controllers::{ParticleEffect, SoundEffect};

/// The RP animation that changes an actors models, or molang data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActorAnimations {
    pub format_version: Version,
    pub animations: HashMap<AnimationIdentifier, Animation>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Animation {
    /// How does time pass when playing the animation. Defaults to `query.anim_time + query.delta_time` which means advance in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anim_time_update: Option<molang::Number>,

    /// Override calculated value (set as the last keyframe time) and set animation length in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation_length: Option<serde_json::Number>,

    /// default = "1.0".  How much this animation is blended with the others.  0.0 = off.  1.0 = fully apply all transforms.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blend_weight: Option<molang::Number>,

    /// Should this animation stop, loop, or stay on the last frame when finished (true, false, hold_on_last_frame).
    #[serde(skip_serializing_if = "Option::is_none", rename = "loop")]
    pub _loop: Option<LoopType>,

    /// How long to wait in seconds before looping this animation. Note that this expression is evaluated after each loop and on looping animation only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loop_delay: Option<molang::Number>,

    /// Reset bones in this animation to the default pose before applying this animation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_previous_animation: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub particle_effects: Option<Timeline<ArrayOrSingle<ParticleEffect>>>,

    /// How long to wait in seconds before playing this animation. Note that this expression is evaluated once before playing, and only re-evaluated if asked to play from the beginning again. A looping animation should use `loop_delay` if it wants a delay between loops.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_delay: Option<molang::Number>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sound_effects: Option<Timeline<ArrayOrSingle<SoundEffect>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeline: Option<Timeline<ArrayOrSingle<VariableDefinition>>>,

    // TODO make sure key is lowercase
    /// Defines how the bones in an animation move or transform.
    pub bones: HashMap<String, Bone>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bone {
    /// The Position transformation during this animation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<BoneFieldType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation: Option<BoneFieldType>,

    /// If set, makes the bone rotation relative to the entity instead of the bone's parent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_to: Option<Relative>,

    /// The Scale transformation during this animation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<BoneFieldType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Relative {
    /// If set, makes the bone rotation relative to the entity instead of the bone's parent.
    pub rotation: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BonePart {
    Uniform(BoneNumericType),
    /// ### As Position<br>
    /// An array of 3 items that describe the bones position.<br>
    /// The position over the X-axis or forwards/backwards.<br>
    /// The position over the Y-axis, or up/down.<br>
    /// The position over the Z-axis, or left/right.<br>
    /// ### As Rotation<br>
    /// An array of 3 items that describe the bones rotation.<br>
    /// The rotation over the X-axis, or up or down.<br>
    /// The rotation over the Y-axis, or yaw.<br>
    /// The rotation over the Z-axis, or roll.<br>
    /// ### As Scale<br>
    /// An array of 3 items that describe the bones scale.<br>
    /// These are undocumented.
    Vector3(BoneNumericType, BoneNumericType, BoneNumericType),
    UniformVector([BoneNumericType; 1]),
    Timeline {
        #[serde(skip_serializing_if = "Option::is_none")]
        lerp_mode: Option<LerpMode>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pre: Option<[BoneNumericType; 3]>,
        post: [BoneNumericType; 3],
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BoneFieldType {
    BonePart(BonePart),
    /// Use String when testing, but StringNumber is valid
    Timeline(Timeline<BonePart>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LerpMode {
    #[serde(rename = "linear")]
    Linear,
    #[serde(rename = "catmullrom")]
    CatmullRom,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BoneNumericType {
    Number(molang::Number),
    X { x: molang::Number },
    Y { y: molang::Number },
    Z { z: molang::Number },
}

/// Should this animation stop, loop, or stay on the last frame when finished.
#[derive(Debug, Clone, PartialEq)]
pub enum LoopType {
    True,
    False,
    HoldOnLastFrame,
}

impl Serialize for LoopType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            LoopType::True => serializer.serialize_bool(true),
            LoopType::False => serializer.serialize_bool(false),
            LoopType::HoldOnLastFrame => serializer.serialize_str("hold_on_last_frame"),
        }
    }
}

impl<'de> Deserialize<'de> for LoopType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LoopTypeVisitor;

        impl<'de> de::Visitor<'de> for LoopTypeVisitor {
            type Value = LoopType;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a boolean or a string")
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if v {
                    Ok(LoopType::True)
                } else {
                    Ok(LoopType::False)
                }
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match v {
                    "hold_on_last_frame" => Ok(LoopType::HoldOnLastFrame),
                    _ => Err(E::custom(format!("invalid loop type: {}", v))),
                }
            }
        }

        deserializer.deserialize_any(LoopTypeVisitor)
    }
}
