
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq)]
pub struct QuerySelection {
    selector:BasicSelection,
    queries:Query,
}
//TODO: Implement Serialize for QuerySelection
impl Serialize for QuerySelection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_str("")?;
        return Ok(state);
    }
}






#[derive(Debug, Clone, PartialEq)]
pub struct Query {
    // Coordinates
    x:Option<f64>,
    y:Option<f64>,
    z:Option<f64>,
    // Volume Dimensions
    dx:Option<f64>,
    dy:Option<f64>,
    dz:Option<f64>,
    // Radius
    r:Option<f64>,
    // Minimum Radius
    rm:Option<f64>,
    // Scores
    scores:Option<HashMap<String,i32>>,



}

pub enum Range {
    NotInt(i32),
    Int(i32),
    NotRange(i32,i32),
    Range(i32,i32),
}




#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BasicSelection {
    #[serde(rename = "@a")]
    AllPlayers,
    #[serde(rename = "@r")]
    RandomPlayer,
    #[serde(rename = "@s")]
    SelfPlayer,
    #[serde(rename = "@p")]
    NearestPlayer,
    #[serde(rename = "@e")]
    AllEntities,
    /// Only applicable to dialogue files
    #[serde(rename = "@initiator")]
    DialogueInitiator,
    /// Education Edition only
    #[serde(rename = "@c")]
    PlayersAgent,
    #[serde(rename = "@v")]
    AllAgents,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Selection {
    Basic(BasicSelection),
   // Query(QuerySelection),
    Player(String),
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WildcardSelection{
  //  Query(QuerySelection),
    Basic(BasicSelection),
    Player(String),
    #[serde(rename = "*")]
    Wildcard,
}




impl From <BasicSelection> for Selection {
    fn from(selector: BasicSelection) -> Self {
        Self::Basic(selector)
    }
}
/*
impl From <QuerySelection> for Selection {
    fn from(selector: QuerySelection) -> Self {
        Self::Query(selector)
    }
}

*/

