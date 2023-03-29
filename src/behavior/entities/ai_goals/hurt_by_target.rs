/* Raw contents of hurt_by_target.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.hurt_by_target",
  "type": "object",
  "title": "Hurt By Target",
  "additionalProperties": false,
  "description": "Allows the mob to target another mob that hurts them.",
  "required": [],
  "properties": {
    "priority": {
      "$ref": "./types/priority.json"
    },
    "entity_types": {
      "$ref": "../types/entity_types.json",
      "description": "List of entity types that this mob can target if they hurt their owner.",
      "title": "Entity Types"
    },
    "alert_same_type": {
      "type": "boolean",
      "default": false,
      "description": "If true, nearby mobs of the same type will be alerted about the damage.",
      "title": "Alert Same Type"
    },
    "hurt_owner": {
      "type": "boolean",
      "default": false,
      "description": "If true, the mob will hurt its owner and other mobs with the same owner as itself.",
      "title": "Hurt Owner"
    }
  },
  "examples": [
    {
      "alert_same_type": false,
      "hurt_owner": false
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HurtByTarget {}
