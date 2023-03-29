/* Raw contents of attack.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.attack",
  "type": "object",
  "additionalProperties": false,
  "title": "Attack",
  "description": "Defines an entity's melee attack and any additional effects on it.",
  "required": ["damage"],
  "properties": {
    "damage": {
      "title": "Damage",
      "$ref": "../types/range_number_type.json",
      "description": "Range of the random amount of damage the melee attack deals. A negative value can heal the entity instead of hurting it."
    },
    "effect_name": {
      "type": "string",
      "description": "Identifier of the status ailment to apply to an entity attacked by this entity's melee attack.",
      "examples": ["wither", "hunger"],
      "title": "Effect Name"
    },
    "effect_duration": {
      "type": "number",
      "default": 1,
      "description": "Duration in seconds of the status ailment applied to the damaged entity.",
      "title": "Effect Duration"
    }
  },
  "examples": [{ "damage": 1 }, { "damage": 1, "effect_name": "example", "effect_duration": 1 }]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Attack {}