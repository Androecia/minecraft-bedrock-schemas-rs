/// A version that tells minecraft what type of data format can be expected when reading this file when used as a format version.
/// can either be a string or a vector of u32 with a length of 3.
use serde::{Deserialize, Serialize};
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Version {
    String(String),
    Array(u32, u32, u32),
}

impl ToString for Version {
    fn to_string(&self) -> String {
        match self {
            Self::String(s) => s.clone(),
            Self::Array(major, minor, patch) => format!("{}.{}.{}", major, minor, patch),
        }
    }
}

impl From<&str> for Version {
    fn from(s: &str) -> Self {
        Self::String(s.to_string())
    }
}

impl From<String> for Version {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}

impl From<&[u32]> for Version {
    fn from(v: &[u32]) -> Self {
        Self::Array(v[0], v[1], v[2])
    }
}

impl From<&Vec<u32>> for Version {
    fn from(v: &Vec<u32>) -> Self {
        Self::Array(v[0], v[1], v[2])
    }
}

impl From<(u32, u32, u32)> for Version {
    fn from(t: (u32, u32, u32)) -> Self {
        Self::Array(t.0, t.1, t.2)
    }
}

impl From<Version> for String {
    fn from(v: Version) -> Self {
        match v {
            Version::String(s) => s,
            Version::Array(major, minor, patch) => format!("{}.{}.{}", major, minor, patch),
        }
    }
}

impl From<Version> for Vec<u32> {
    fn from(v: Version) -> Self {
        match v {
            Version::String(s) => {
                let mut split = s.split(".");
                let major = split.next().unwrap().parse().unwrap();
                let minor = split.next().unwrap().parse().unwrap();
                let patch = split.next().unwrap().parse().unwrap();
                vec![major, minor, patch]
            }
            Version::Array(major, minor, patch) => vec![major, minor, patch],
        }
    }
}
