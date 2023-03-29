use crate::general::identifier::IdentifierError;
use crate::utils::Validation;
use regex::Regex;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct AnimationIdentifier(String);

impl TryFrom<String> for AnimationIdentifier {
    type Error = IdentifierError;
    fn try_from(s: String) -> std::result::Result<Self, Self::Error> {
        if AnimationIdentifier::validate(&s) {
            Ok(Self(s[10..].to_string()))
        } else {
            Err(IdentifierError::InvalidIdentifier(s))
        }
    }
}

impl Validation<&str> for AnimationIdentifier {
    fn validate(s: &str) -> bool {
        let re = Regex::new(r"^animation\.[a-z.]+").unwrap();
        re.is_match(&s)
    }
}

impl std::fmt::Display for AnimationIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "animation.{}", self.0)
    }
}

impl Serialize for AnimationIdentifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for AnimationIdentifier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if AnimationIdentifier::validate(&s) {
            Ok(Self(s[10..].to_string()))
        } else {
            Err(serde::de::Error::custom(format!(
                "expected animation identifier but got {}",
                s
            )))
        }
    }
}
