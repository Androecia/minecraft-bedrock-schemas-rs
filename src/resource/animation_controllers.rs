use crate::general::identifier::animation_controller::AnimationControllerIdentifier;
use crate::general::version::Version;
use crate::molang::{self, Molang};
pub use crate::shared::animation_controllers::*;
use crate::types::Timeline;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AnimationControllers {
    pub format_version: Version,
    pub animation_controllers: HashMap<AnimationControllerIdentifier, AnimationController>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ParticleEffect {
    /// Set to false to have the effect spawned in the world without being bound to an actor (by default an effect is bound to the actor).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bind_to_actor: Option<bool>,

    /// The name of a particle effect that should be played.
    pub effect: String,

    /// The name of a locator on the actor where the effect should be located.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locator: Option<String>,

    /// A molang script that will be run when the particle emitter is initialized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_effect_script: Option<Molang>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AnimationController {
    // Part of me feels like making this an this into a vector may improve performance
    /// The states of this animation controller.
    pub states: HashMap<String, AnimationState>,

    /// The state to start with, if not specified state at position 0 in the array is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_state: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AnimationState {
    /// The animations definition for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animations: Option<Vec<Animation>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub blend_transition: Option<BlendTransition>,

    /// When blending a transition to another state, animate each euler axis through the shortest rotation, instead of by value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blend_via_shortest_path: Option<bool>,

    /// The effects to be emitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub particle_effects: Option<Vec<ParticleEffect>>,

    /// Collection of sounds to trigger on entry to this animation state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sound_effects: Option<Vec<SoundEffect>>,

    /// Events, commands or transitions to preform on entry of this state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_entry: Option<Vec<Molang>>,

    /// Events, commands or transitions to preform on exit of this state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_exit: Option<Vec<Molang>>,

    /// The transition definition for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transitions: Option<Vec<Animation>>,

    /// Sets molang on data on entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<Variables>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Variables {
    pub input: molang::Number,
    pub remap_curve: Timeline<serde_json::Number>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum BlendTransition {
    // A short-hand version of blend_out that simply sets the amount of time to fade out if the animation is interrupted. Minimum is 0
    BlendTime(f64),

    /// Specifies the cross-fade time in seconds when transitioning to another state.
    BlendCurve(HashMap<String, f64>),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SoundEffect {
    /// Valid sound effect names should be listed in the entity's resource_definition json file.
    pub effect: String,
}
