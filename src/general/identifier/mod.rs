




pub mod animation_controller;

pub mod namespace;


pub enum Identifier {
    AnimationController(animation_controller::AnimationControllerIdentifier),
    Namespace(namespace::NamespaceIdentifier),
}

#[derive(Debug)]
pub enum IdentifierError {
    InvalidIdentifier(String),
}


impl std::error::Error for IdentifierError {

}


impl std::fmt::Display for IdentifierError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IdentifierError::InvalidIdentifier(s) => write!(f, "Invalid identifier: {}", s),
        }
    }
}
