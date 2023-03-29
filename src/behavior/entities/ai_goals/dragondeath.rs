/* Raw contents of dragondeath.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.dragondeath",
  "additionalProperties": false,
  "type": "object",
  "title": "Dragondeath",
  "description": "Controls the entity's death state and animation. Note: This behavior can only be used by the ender_dragon entity type.",
  "properties": {
    "priority": {
      "$ref": "./types/priority.json"
    }
  }
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Dragondeath {}
