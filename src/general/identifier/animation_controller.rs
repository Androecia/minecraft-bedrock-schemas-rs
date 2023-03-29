use crate::general::identifier::IdentifierError;
use crate::utils::Validation;
use regex::Regex;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct AnimationControllerIdentifier(String);

impl TryFrom<String> for AnimationControllerIdentifier {
    type Error = IdentifierError;
    fn try_from(s: String) -> std::result::Result<Self, Self::Error> {
        if AnimationControllerIdentifier::validate(&s) {
            Ok(Self(s[21..].to_string()))
        } else {
            Err(IdentifierError::InvalidIdentifier(s))
        }
    }
}

impl Validation<&str> for AnimationControllerIdentifier {
    fn validate(s: &str) -> bool {
        let re = Regex::new(r"^controller\.animation\.[a-z.]+").unwrap();
        re.is_match(&s)
    }
}

impl std::fmt::Display for AnimationControllerIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "controller.animation.{}", self.0)
    }
}

impl Serialize for AnimationControllerIdentifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for AnimationControllerIdentifier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if AnimationControllerIdentifier::validate(&s) {
            Ok(Self(s[21..].to_string()))
        } else {
            Err(serde::de::Error::custom(format!(
                "expected animation controller identifier but got {}",
                s
            )))
        }
    }
}
