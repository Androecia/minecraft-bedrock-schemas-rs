use std::collections::HashMap;
use std::convert::TryInto;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

pub type Timeline<T> = HashMap<StringNumber, T>;

// TODO: Switch to serde_json::Number at some point and figure out how to preserve 0 for testing
/// serde::Number that serializes as a string
#[derive(Debug, Clone, PartialEq, Hash, Eq, Serialize, Deserialize)]
pub struct StringNumber(String);

impl TryInto<serde_json::Number> for StringNumber {
    type Error = serde_json::Error;
    fn try_into(self) -> Result<serde_json::Number, Self::Error> {
        serde_json::Number::from_str(&self.0)
    }
}

impl From<serde_json::Number> for StringNumber {
    fn from(num: serde_json::Number) -> Self {
        Self(num.to_string())
    }
}

// TODO: Better name for this
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ArrayOrSingle<T> {
    Single(T),
    Array(Vec<T>),
}
