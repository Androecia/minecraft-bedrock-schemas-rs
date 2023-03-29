/* Raw contents of tameable.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.tameable",
  "type": "object",
  "title": "Tameable",
  "description": "Defines the rules for a mob to be tamed by the player.",
  "required": [],
  "additionalProperties": false,
  "properties": {
    "probability": {
      "type": "number",
      "default": 1,
      "description": "The chance of taming the entity with each item use between 0.0 and 1.0, where 1.0 is 100%",
      "minimum": 0,
      "maximum": 1,
      "title": "Probability"
    },
    "tame_event": {
      "title": "Tame Event",
      "$ref": "../types/event_object.json",
      "description": "Event to run when this entity becomes tamed."
    },
    "tame_items": {
      "title": "Tame Items",
      "description": "The list of items that can be used to tame this entity.",
      "oneOf": [
        {
          "type": "array",
          "items": {
            "$ref": "../../../../general/item/identifier.json"
          }
        },
        {
          "$ref": "../../../../general/item/identifier.json"
        }
      ]
    }
  },
  "examples": [
    {
      "probability": 1
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tameable {}
