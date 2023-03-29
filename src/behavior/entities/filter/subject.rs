use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Subject {
    /// The block involved with the interaction.
    #[serde(rename = "block")]
    Block,
    /// The damaging actor involved with the interaction.
    #[serde(rename = "damager")]
    Damager,
    /// The other member of an interaction, not the caller.
    #[serde(rename = "other")]
    Other,
    /// The caller's current parent.
    #[serde(rename = "parent")]
    Parent,
    /// The player involved with the interaction.
    #[serde(rename = "player")]
    Player,
    /// The entity or object calling the test
    #[serde(rename = "self")]
    Self_,
    /// The caller's current target.
    #[serde(rename = "target")]
    Target,
}

impl Default for Subject {
    fn default() -> Self {
        Subject::Self_
    }
}
