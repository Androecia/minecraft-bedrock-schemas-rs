/* Raw contents of item_controllable.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.item_controllable",
  "description": "Efines what items can be used to control this entity while ridden.",
  "type": "object",
  "title": "Item Controllable",
  "additionalProperties": false,
  "required": [],
  "properties": {
    "control_items": {
      "description": "List of items that can be used to control this entity.",
      "oneOf": [
        {
          "type": "array",
          "items": {
            "type": "string",
            "description": "An item that can be used to control this entity.",
            "title": "Item"
          }
        },
        {
          "type": "string"
        }
      ],
      "title": "Control Items"
    }
  }
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemControllable {}
