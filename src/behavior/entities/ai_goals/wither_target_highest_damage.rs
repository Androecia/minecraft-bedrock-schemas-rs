/* Raw contents of wither_target_highest_damage.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.wither_target_highest_damage",
  "description": "Allows the wither to focus its attacks on whichever mob has dealt the most damage to it. Can only be used by the Wither Boss.",
  "additionalProperties": false,
  "type": "object",
  "title": "Wither Target Highest Damage",
  "required": [],
  "properties": {
    "priority": {
      "$ref": "./types/priority.json"
    },
    "entity_types": {
      "$ref": "../types/entity_types.json",
      "description": "List of entity types the wither takes into account to find who dealt the most damage to it.",
      "title": "Entity Types"
    }
  }
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WitherTargetHighestDamage {}
