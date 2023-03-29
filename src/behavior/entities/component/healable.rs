/* Raw contents of healable.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.healable",
  "description": "Defines the interactions with this entity for healing it.",
  "type": "object",
  "title": "Healable",
  "additionalProperties": false,
  "required": [],
  "definitions": {
    "effect": {
      "type": "object",
      "properties": {
        "name": {
          "title": "Name",
          "description": "UNDOCUMENTED.",
          "$comment": "UNDOCUMENTED",
          "type": "string"
        },
        "duration": {
          "type": "integer",
          "default": 0,
          "minimum": 0,
          "description": "The duration of the effect.",
          "$comment": "UNDOCUMENTED"
        },
        "amplifier": {
          "type": "integer",
          "default": 0,
          "minimum": 0,
          "description": "The amplifier of the effect.",
          "$comment": "UNDOCUMENTED"
        }
      }
    }
  },
  "properties": {
    "filters": {
      "$ref": "../../filters/filters.json"
    },
    "force_use": {
      "type": "boolean",
      "default": false,
      "description": "Determines if item can be used regardless of entity being at full health.",
      "title": "Force Use"
    },
    "items": {
      "description": "The array of items that can be used to heal this entity.",
      "title": "Items",
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "filters": {
            "$ref": "../../filters/filters.json",
            "description": "The filter group that defines the conditions for using this item to heal the entity."
          },
          "heal_amount": {
            "type": "integer",
            "default": 1,
            "description": "The amount of health this entity gains when fed this item.",
            "title": "Heal Amount"
          },
          "item": {
            "$ref": "../../../../general/item/descriptor.json",
            "description": "Item that can be used to heal this entity.",
            "title": "Item"
          },
          "effects": {
            "oneOf": [
              {
                "$ref": "#/definitions/effect"
              },
              {
                "type": "array",
                "items": {
                  "$ref": "#/definitions/effect"
                }
              }
            ]
          }
        }
      }
    }
  },
  "examples": [
    {
      "force_use": false,
      "items": []
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Healable {}
