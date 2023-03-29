/* Raw contents of charge_held_item.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.charge_held_item",
  "type": "object",
  "title": "Charge Held Item",
  "description": "Allows an entity to charge and use their held item.",
  "additionalProperties": false,
  "required": [],
  "properties": {
    "priority": {
      "$ref": "./types/priority.json"
    },
    "items": {
      "type": "array",
      "title": "Items",
      "description": "The list of items that can be used to charge the held item. This list is required and must have at least one item in it.",
      "items": {
        "$ref": "../../../../general/item/descriptor.json",
        "description": "Items names to be used.",
        "title": "Item ID"
      }
    }
  },
  "examples": [
    {
      "items": []
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChargeHeldItem {}
