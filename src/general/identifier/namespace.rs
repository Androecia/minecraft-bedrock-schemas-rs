use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, marker::PhantomData};

use crate::{
    behavior::{
        block::Block,
        entities::{event::Event, Entities},
        item::Item,
    },
    utils::Validation,
};

use super::IdentifierError;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct NamespaceIdentifier<T>(String, PhantomData<T>);

impl<T> Serialize for NamespaceIdentifier<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.0.as_str())
    }
}

impl From<&str> for NamespaceIdentifier<Block> {
    fn from(s: &str) -> Self {
        Self::try_from(s.to_string()).unwrap()
    }
}

impl<T> NamespaceIdentifier<T> {
    pub fn new(s: String) -> Self {
        Self(s, PhantomData)
    }

    pub fn get(&self) -> &String {
        &self.0
    }

    pub fn set(&mut self, s: String) {
        self.0 = s;
    }
}

impl<'de, T> Deserialize<'de> for NamespaceIdentifier<T> {
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

impl<T> Validation<&str> for NamespaceIdentifier<T> {
    fn validate(s: &str) -> bool {
        //FIXME
        //println!("s = {}", s);

        // let re = Regex::new(r"^[:_-.0-9a-zA-Z]+$").unwrap();
        //re.is_match(&s)

        true
    }
}

impl<T> Display for NamespaceIdentifier<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T> TryFrom<String> for NamespaceIdentifier<T> {
    type Error = IdentifierError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let s = s.trim();

        if !Self::validate(s) {
            return Err(IdentifierError::InvalidIdentifier(s.to_string()));
        }
        Ok(Self(s.to_string(), PhantomData))
    }
}
