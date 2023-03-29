/* Raw contents of vex_random_move.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.vex_random_move",
  "description": "Allows the mob to move around randomly like the Vex.",
  "additionalProperties": false,
  "type": "object",
  "title": "Vex Random Move",
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
pub struct VexRandomMove {}
