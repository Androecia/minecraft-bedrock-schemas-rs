


/// A version that tells minecraft what type of data format can be expected when reading this file when used as a format version.
/// can either be a string or a vector of u32 with a length of 3.
pub struct FormatVersion {
    major:u32,
    minor:u32,
    patch:u32,
}

use serde::{Deserialize, Serialize};


// ! FIX this too allow for deserialization of a vector of u32 and a string



/// Serialize and Deserialize as a vector of u32 and as a string.

impl Serialize for FormatVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }

}

impl<'de> Deserialize<'de> for FormatVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
       // check if the version is a string or a vector of u32
            let s = String::deserialize(deserializer)?;
            Ok(Self::from(s))
    }
}










impl FormatVersion {
    pub fn new (major:u32, minor:u32, patch:u32) -> Self {
        Self { major, minor, patch }
    }
}

impl ToString for FormatVersion {
    fn to_string (&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl FromStr for FormatVersion {
    type Err = ();

    fn from_str (s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(".");
        let major = split.next().unwrap().parse().unwrap();
        let minor = split.next().unwrap().parse().unwrap();
        let patch = split.next().unwrap().parse().unwrap();
        Ok(Self { major, minor, patch })
    }
}





impl From <&str> for FormatVersion {
    fn from (s:&str) -> Self {
        let mut split = s.split(".");
        let major = split.next().unwrap().parse().unwrap();
        let minor = split.next().unwrap().parse().unwrap();
        let patch = split.next().unwrap().parse().unwrap();
        Self { major, minor, patch }
    }
}

impl From<String> for FormatVersion {
    fn from (s:String) -> Self {
        Self::from(s.as_str())
    }
}



impl From <&[u32]> for FormatVersion {
    fn from (v:&[u32]) -> Self {
        Self { major: v[0], minor: v[1], patch: v[2] }
    }
}

impl From <&Vec<u32>> for FormatVersion {
    fn from (v:&Vec<u32>) -> Self {
        Self { major: v[0], minor: v[1], patch: v[2] }
    }
}

impl Into<Vec<u32>> for FormatVersion {
    fn into (self) -> Vec<u32> {
        vec![self.major, self.minor, self.patch]
    }
}

impl Into<String> for FormatVersion {
    fn into (self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
}


impl PartialEq for FormatVersion {
    fn eq (&self, other: &Self) -> bool {
        self.major == other.major && self.minor == other.minor && self.patch == other.patch
    }
}

impl PartialOrd for FormatVersion {
    fn partial_cmp (&self, other: &Self) -> Option<std::cmp::Ordering> {

        if self.major > other.major {
            Some(std::cmp::Ordering::Greater)
        } else if self.major < other.major {
            Some(std::cmp::Ordering::Less)
        } else if self.minor > other.minor {
            Some(std::cmp::Ordering::Greater)
        } else if self.minor < other.minor {
            Some(std::cmp::Ordering::Less)
        } else if self.patch > other.patch {
            Some(std::cmp::Ordering::Greater)
        } else if self.patch < other.patch {
            Some(std::cmp::Ordering::Less)
        } else {
            Some(std::cmp::Ordering::Equal)
        }

    }
}

impl Eq for FormatVersion {}

impl ToString for FormatVersion {
    fn to_string (&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl FormatVersion {
    fn new (major:u32, minor:u32, patch:u32) -> Self {
        Self { major, minor, patch }
    }
}
