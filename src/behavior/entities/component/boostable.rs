/* Raw contents of boostable.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.boostable",
  "type": "object",
  "title": "Boostable",
  "additionalProperties": false,
  "description": "Defines the conditions and behavior of a rideable entity's boost.",
  "properties": {
    "duration": {
      "type": "number",
      "default": 3,
      "description": "Time in seconds for the boost.",
      "title": "Duration"
    },
    "speed_multiplier": {
      "type": "number",
      "default": 1,
      "description": "Factor by which the entity's normal speed increases. E.g. 2.0 means go twice as fast.",
      "title": "Speed Multiplier"
    },
    "boost_items": {
      "type": "array",
      "description": "List of items that can be used to boost while riding this entity.",
      "title": "Boost Items",
      "items": {
        "type": "object",
        "additionalProperties": false,
        "description": "List of items that can be used to boost while riding this entity.",
        "properties": {
          "damage": {
            "type": "integer",
            "default": 1,
            "description": "This is the damage that the item will take each time it is used.",
            "title": "Damage"
          },
          "item_damage": {
            "type": "integer",
            "default": 1,
            "description": "UNDOCUMENTED.",
            "title": "Item Damage"
          },
          "item": {
            "$ref": "../../../../general/item/descriptor.json",
            "default": "",
            "description": "Name of the item that can be used to boost.",
            "title": "Item"
          },
          "replace_item": {
            "$ref": "../../../../general/item/descriptor.json",
            "default": "",
            "description": "The item used to boost will become this item once it is used up.",
            "title": "Replace Item"
          }
        }
      }
    }
  },
  "examples": [
    {
      "duration": 3,
      "speed_multiplier": 1,
      "boost_items": []
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Boostable {}
