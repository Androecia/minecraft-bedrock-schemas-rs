/* Raw contents of type_family.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.type_family",
  "type": "object",
  "title": "Type Family",
  "additionalProperties": false,
  "description": "Defines the families this entity belongs to.",
  "required": ["family"],
  "examples": [{ "family": [] }, { "family": ["monster"] }, { "family": ["mob"] }, { "family": ["animal"] }, { "family": ["npc"] }],
  "properties": {
    "family": {
      "type": "array",
      "items": { "type": "string", "description": "Family name.", "title": "Family", "examples": ["monster", "mob", "animal", "npc"] },
      "description": "List of family names.",
      "title": "Family"
    }
  }
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypeFamily {}
