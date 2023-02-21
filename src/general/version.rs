


/// A version that tells minecraft what type of data format can be expected when reading this file when used as a format version.
///
pub struct Version {
    major:u32,
    minor:u32,
    patch:u32,
}

impl From <&str> for Version {
    fn from (s:&str) -> Self {
        let mut split = s.split(".");
        let major = split.next().unwrap().parse().unwrap();
        let minor = split.next().unwrap().parse().unwrap();
        let patch = split.next().unwrap().parse().unwrap();
        Self { major, minor, patch }
    }
}

impl From<String> for Version {
    fn from (s:String) -> Self {
        Self::from(s.as_str())
    }
}

impl From (u32,u32,u32) for Version {
    fn from (t:(u32,u32,u32)) -> Self {
        Self { major: t.0, minor: t.1, patch: t.2 }
    }
}

impl From <&[u32]> for Version {
    fn from (v:&[u32]) -> Self {
        Self { major: v[0], minor: v[1], patch: v[2] }
    }
}

impl From <&Vec<u32>> for Version {
    fn from (v:&Vec<u32>) -> Self {
        Self { major: v[0], minor: v[1], patch: v[2] }
    }
}

impl Into<Vec<u32>> for Version {
    fn into (self) -> Vec<u32> {
        vec![self.major, self.minor, self.patch]
    }
}

impl Into<String> for Version {
    fn into (self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl Into<&[u32]> for Version {
    fn into (self) -> &[u32] {
        &vec![self.major, self.minor, self.patch]
    }
}

impl PartialEq for Version {
    fn eq (&self, other: &Self) -> bool {
        self.major == other.major && self.minor == other.minor && self.patch == other.patch
    }
}

impl PartialOrd for Version {
    fn partial_cmp (&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_vec:Vec<u32> = self.into();
        let other_vec:Vec<u32> = other.into();
        self_vec.partial_cmp(&other_vec)
    }
}

impl Eq for Version {}

impl ToString for Version {
    fn to_string (&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl Version {
    fn new (major:u32, minor:u32, patch:u32) -> Self {
        Self { major, minor, patch }
    }
}
