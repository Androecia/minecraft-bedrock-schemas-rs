/* Raw contents of owner_hurt_by_target.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.owner_hurt_by_target",
  "type": "object",
  "title": "Owner Hurt By Target",
  "additionalProperties": false,
  "description": "Allows the mob to target another mob that hurts their owner.",
  "required": [],
  "properties": {
    "priority": {
      "$ref": "./types/priority.json"
    },
    "entity_types": {
      "$ref": "../types/entity_types.json",
      "description": "List of entity types that this mob can target if they hurt their owner.",
      "title": "Entity Types"
    }
  }
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OwnerHurtByTarget {}
