/* Raw contents of giveable.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.giveable",
  "additionalProperties": false,
  "description": "Defines sets of items that can be used to trigger events when used on this entity. The item will also be taken and placed in the entity's inventory.",
  "type": "object",
  "title": "Giveable",
  "properties": {
    "triggers": {
      "description": "Defines sets of items that can be used to trigger events when used on this entity. The item will also be taken and placed in the entity's inventory.",
      "title": "Triggers",
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "cooldown": {
          "type": "number",
          "default": 0.0,
          "description": "An optional cool down in seconds to prevent spamming interactions.",
          "title": "Cooldown"
        },
        "items": {
          "type": "array",
          "title": "Properties",
          "description": "The list of items that can be given to the entity to place in their inventory.",
          "items": {
            "description": "An items that can be given to the entity to place in their inventory.",
            "$ref": "../../../../general/item/descriptor.json",
            "title": "Properties"
          }
        },
        "on_give": {
          "$ref": "../types/event_object.json",
          "description": "Event to fire when the correct item is given.",
          "title": "On Give"
        }
      }
    }
  },
  "examples": [
    {
      "triggers": {}
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Giveable {}
