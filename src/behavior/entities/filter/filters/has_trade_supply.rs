/* Raw contents of has_trade_supply.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.filters.has_trade_supply",
  "type": "object",
  "title": "Has Trade Supply",
  "description": "Tests whether the target has any trade supply left. Will return false if the target cannot be traded with.",
  "properties": {
    "test": {
      "type": "string",
      "title": "Test Property",
      "description": "Tests whether the target has any trade supply left. Will return false if the target cannot be traded with."
    },
    "operator": {
      "$ref": "./types/operator.json"
    },
    "subject": {
      "$ref": "./types/subject.json"
    },
    "value": {
      "title": "Value",
      "description": "True or false.",
      "type": "boolean",
      "default": true
    }
  },
  "examples": [
    {
      "test": "has_trade_supply",
      "value": true
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HasTradeSupply {}
