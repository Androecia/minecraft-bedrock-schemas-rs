/// if the molang is None then the output of the animation is the name of the animation itself. If the molang is Some then the output of the animation is the result of the molang in a key that is the name and the value which is the molang as a string.
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use crate::molang::Molang;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum Animation {
    String(String),
    Molang(HashMap<String, Molang>),
}
