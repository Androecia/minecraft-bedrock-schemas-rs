/* Raw contents of trade_interest.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.trade_interest",
  "description": "Allows the mob to look at a player that is holding a tradable item.",
  "type": "object",
  "title": "Trade Interest",
  "additionalProperties": false,
  "required": [],
  "properties": {
    "priority": {
      "$ref": "./types/priority.json"
    },
    "carried_item_switch_time": {
      "type": "number",
      "default": 2,
      "description": "The Maximum time in seconds that the trader will hold an item before attempting to switch for a different item that takes the same trade.",
      "title": "Carried Item Switch Time"
    },
    "cooldown": {
      "type": "number",
      "default": 2,
      "description": "The time in seconds before the trader can use this goal again.",
      "title": "Cooldown"
    },
    "interest_time": {
      "type": "number",
      "default": 45,
      "description": "The Maximum time in seconds that the trader will be interested with showing it's trade items.",
      "title": "Interest_time"
    },
    "remove_item_time": {
      "type": "number",
      "default": 1,
      "description": "The Maximum time in seconds that the trader will wait when you no longer have items to trade.",
      "title": "Remove Item Time"
    },
    "within_radius": {
      "type": "number",
      "default": 0,
      "description": "Distance in blocks this mob can be interested by a player holding an item they like.",
      "title": "Within Radius"
    }
  },
  "examples": [
    {
      "carried_item_switch_time": 2,
      "cooldown": 2,
      "interest_time": 45,
      "remove_item_time": 1,
      "within_radius": 0
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TradeInterest {}
