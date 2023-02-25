
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Identifier {
    namespace: String,
    path: String,
}

use std::fmt::Display;

use serde::{Deserialize, Serialize};

impl Serialize for Identifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {



        serializer.serialize_str(&self.to_string())
    }
}

use regex::Regex;
impl<'de> Deserialize<'de> for Identifier {
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


impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.namespace, self.path)
    }
}



impl TryFrom <String> for Identifier {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let s = s.trim();

        let re = Regex::new(r"^[a-zA-Z0-9_]+:[a-zA-Z0-9_]+$").unwrap();

        if !re.is_match(&s) {
            return Err(format!("Invalid identifier: {}", s));
        }


        let mut split = s.split(":");
        let namespace = split.next().unwrap().to_string();

        if namespace.is_empty() {
            return Err(format!("Invalid identifier: {}", s));
        }

        let path = split.next().unwrap().to_string();

        if path.is_empty() {
            return Err(format!("Invalid identifier: {}", s));
        }

        Ok(Self { namespace, path })
    }
}




