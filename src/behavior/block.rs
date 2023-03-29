use std::collections::HashMap;

use crate::general::identifier::namespace::NamespaceIdentifier;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Block;
pub type BlockIdentifier = NamespaceIdentifier<Block>;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
// TODO: Typings for the vanilla block states, they are found in the vanilla data generated with test_config.json, test where else this is valid when using an object as a block identifier for now I will just keep it separate
pub struct BlockStateIdentifier {
    name: BlockIdentifier,

    states: Option<HashMap<String, String>>,
}
// TODO: when typings are fixed, this should be a BlockStateIdentifier depending on findings in the future
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MaterialIdentifier {
    StateIdentifier(BlockStateIdentifier),

    Identifier(BlockIdentifier),
}
