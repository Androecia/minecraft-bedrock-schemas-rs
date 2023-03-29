/* Raw contents of barter.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.barter",
  "type": "object",
  "title": "Barter",
  "description": "Enables the component to drop an item as a barter exchange.",
  "additionalProperties": false,
  "properties": {
    "barter_table": {
      "type": "string",
      "description": "Loot table that's used to drop a random item.",
      "title": "Barter Table"
    },
    "cooldown_after_being_attacked": {
      "type": "integer",
      "default": 0,
      "description": "Duration, in seconds, for which mob won't barter items if it was hurt.",
      "title": "Cooldown After Being Attacked"
    }
  },
  "examples": [
    {
      "barter_table": "example",
      "cooldown_after_being_attacked": 0
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Barter {}
