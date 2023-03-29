/* Raw contents of physics.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.physics",
  "additionalProperties": false,
  "type": "object",
  "title": "Physics",
  "description": "Defines physics properties of an actor, including if it is affected by gravity or if it collides with objects.",
  "required": [],
  "properties": {
    "has_collision": {
      "type": "boolean",
      "default": true,
      "description": "Whether or not the object collides with things.",
      "title": "Has Collision"
    },
    "has_gravity": {
      "type": "boolean",
      "default": true,
      "description": "Whether or not the entity is affected by gravity.",
      "title": "Has Gravity"
    }
  },
  "examples": [{}, { "has_collision": true, "has_gravity": true }]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Physics {}
