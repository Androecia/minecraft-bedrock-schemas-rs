use std::collections::HashMap;

use crate::{
    general::{identifier::animation::AnimationIdentifier, version::Version},
    types::Timeline,
};
use serde::{Deserialize, Serialize};

use super::animation_controllers::Executer;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

pub struct Animations {
    /// The format version of the file.
    pub format_version: Version,
    /// The animation specification.
    pub animations: HashMap<AnimationIdentifier, Animation>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Animation {
    /// The time in seconds this animation will last.
    pub animation_length: f64,
    /// Whenever this animation should loop once it reaches the end, will only happen if the animation is still active.
    #[serde(rename = "loop")]
    pub _loop: bool,
    /// A timeline specification, property names are timestamps.
    pub timeline: Timeline<TimelineItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TimelineItem {
    String(Executer),
    Collection(Vec<Executer>),
}
