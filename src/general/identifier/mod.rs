use regex::Regex;

use crate::{
    behavior::{
        block::Block,
        entities::{event::Event, Entities},
        item::Item,
    },
    utils::Validation,
};

use self::namespace::NamespaceIdentifier;

pub mod animation;
pub mod animation_controller;
pub mod namespace;

pub enum Identifier {
    AnimationController(animation_controller::AnimationControllerIdentifier),
    Animation(animation::AnimationIdentifier),
    Block(Identifier<Block>),
    Item(Identifier<Item>),
    Entity(Identifier<Entities>),
    Event(Identifier<Event>),
}

#[derive(Debug)]
pub enum IdentifierError {
    InvalidIdentifier(String),
}

impl std::error::Error for IdentifierError {}

impl std::fmt::Display for IdentifierError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IdentifierError::InvalidIdentifier(s) => write!(f, "Invalid identifier: {}", s),
        }
    }
}
