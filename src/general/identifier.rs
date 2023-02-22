
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Identifier {
    namespace: String,
    path: String,
}

use serde::{Deserialize, Serialize};

impl Serialize for Identifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Identifier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Self::from(s))
    }
}




impl ToString for Identifier {
    fn to_string(&self) -> String {
        format!("{}:{}", self.namespace, self.path)
    }
}

impl From<&str> for Identifier {
    fn from(s: &str) -> Self {
        let mut split = s.split(":");
        let namespace = split.next().unwrap().to_string();
        let path = split.next().unwrap().to_string();
        Self { namespace, path }
    }
}

impl From<String> for Identifier {
    fn from(s: String) -> Self {
        Self::from(s.as_str())
    }
}

impl Into <String> for Identifier {
    fn into (self) -> String {
        self.to_string()
    }
}


