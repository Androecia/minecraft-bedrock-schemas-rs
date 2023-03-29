/* Raw contents of drink_potion.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.drink_potion",
  "type": "object",
  "title": "Drink Potion",
  "description": "Allows the mob to drink potions based on specified environment conditions.",
  "additionalProperties": false,
  "properties": {
    "priority": {
      "$ref": "./types/priority.json"
    },
    "speed_multiplier": {
      "$ref": "./types/speed_multiplier.json"
    },
    "speed_modifier": {
      "default": 0.0,
      "description": "Movement speed modifier of the mob when using this AI Goal.",
      "title": "Speed Modifier"
    },
    "potions": {
      "type": "array",
      "description": "A list of potions that this entity can drink.",
      "title": "Potions",
      "items": {
        "required": ["id", "chance", "filters"],
        "additionalProperties": false,
        "type": "object",
        "description": "A potions that this entity can drink.",
        "title": "Potions",
        "properties": {
          "id": {
            "type": "integer",
            "default": -1,
            "description": "The registry ID of the potion to use.",
            "title": "Id"
          },
          "chance": {
            "type": "number",
            "default": 1.0,
            "minimum": 0,
            "maximum": 1,
            "description": "The percent chance (from 0.0 to 1.0) of this potion being selected when searching for a potion to use.",
            "title": "Chance"
          },
          "filters": {
            "description": "The filters to use when determining if this potion can be selected.",
            "$ref": "../../filters/filters.json"
          }
        }
      }
    }
  },
  "examples": [
    {
      "potions": []
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DrinkPotion {}
