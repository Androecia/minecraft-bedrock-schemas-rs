/* Raw contents of trusting.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.trusting",
  "type": "object",
  "title": "Trusting",
  "description": "Defines the rules for a mob to trust players.",
  "required": [],
  "additionalProperties": false,
  "properties": {
    "probability": {
      "type": "number",
      "default": 1,
      "description": "The chance of the entity trusting with each item use between 0.0 and 1.0, where 1.0 is 100%",
      "title": "Probability"
    },
    "trust_event": {
      "$ref": "../types/event_object.json",
      "description": "Event to run when this entity becomes trusting.",
      "title": "Trust Event"
    },
    "trust_items": {
      "title": "Trust Items",
      "type": "array",
      "description": "The list of items that can be used to get the entity to trust players.",
      "items": {
        "$ref": "../../../../general/item/identifier.json",
        "title": "Trust Item"
      }
    }
  },
  "examples": [
    {
      "probability": 1,
      "trust_event": "self:trust"
    },
    {
      "probability": 1,
      "trust_items": [],
      "trust_event": "self:trust"
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Trusting {}
