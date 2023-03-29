/* Raw contents of vex_copy_owner_target.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.vex_copy_owner_target",
  "description": "Allows the mob to target the same entity its owner is targeting.",
  "additionalProperties": false,
  "type": "object",
  "title": "Vex Copy Owner Target",
  "required": [],
  "properties": {
    "priority": {
      "$ref": "./types/priority.json"
    },
    "entity_types": {
      "$ref": "../types/entity_types.json",
      "description": "List of entities this mob can copy the owner from.",
      "title": "Entity Types"
    }
  }
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VexCopyOwnerTarget {}
