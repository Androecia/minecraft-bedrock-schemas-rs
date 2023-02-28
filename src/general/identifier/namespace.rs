use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::utils::Validation;

use super::IdentifierError;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct NamespaceIdentifier {
    namespace: String,
    path: Vec<String>,
}

impl Serialize for NamespaceIdentifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for NamespaceIdentifier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        match Self::try_from(s) {
            Ok(s) => Ok(s),
            Err(e) => Err(serde::de::Error::custom(e)),
        }
    }
}

impl Validation<&str> for NamespaceIdentifier {
    fn validate(s: &str) -> bool {
        let re = Regex::new(r"^[0-9a-zA-Z:_\\-\\.]+$").unwrap();
        re.is_match(&s)
    }
}

impl Display for NamespaceIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.namespace, self.path.join(":"))
    }
}

impl TryFrom<String> for NamespaceIdentifier {
    type Error = IdentifierError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let s = s.trim();

        if !Self::validate(s) {
            return Err(IdentifierError::InvalidIdentifier(s.to_string()));
        }

        let mut path: Vec<String> = s.split(":").map(|s| s.to_string()).collect::<Vec<String>>();

        let namespace = match path.pop() {
            Some(s) => s.to_string(),
            None => return Err(IdentifierError::InvalidIdentifier(s.to_string())),
        };

        Ok(Self { namespace, path })
    }
}
