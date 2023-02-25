use std::{str::FromStr, fmt::Display};

use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq)]
pub struct Molang {
    script: String,
}

#[derive(Debug)]
pub enum MolangError {
    InvalidMolang(String),
}

impl Display for MolangError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MolangError::InvalidMolang(s) => write!(f, "Invalid Molang: {}", s),
        }
    }
}


use std::error::Error;

impl Error for MolangError {


    fn cause(&self) -> Option<&dyn Error> {
        None
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }



}


impl TryFrom<String> for Molang {
    type Error = MolangError;

    fn try_from(script: String) -> Result<Self, Self::Error> {
        Ok(Self { script })
    }
}


impl ToString for Molang {
    fn to_string(&self) -> String {
        self.script.clone()
    }
}



impl Serialize for Molang {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.script)
    }
}

impl<'de> Deserialize<'de> for Molang {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Self::try_from(s).unwrap())
    }
}
