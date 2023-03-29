/* Raw contents of trade_resupply.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.trade_resupply",
  "description": "Resupplies an entity's trade.",
  "type": "object",
  "title": "Trade Resupply",
  "additionalProperties": false,
  "properties": {}
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TradeResupply {}
