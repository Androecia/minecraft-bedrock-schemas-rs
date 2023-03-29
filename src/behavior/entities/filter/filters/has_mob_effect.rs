/* Raw contents of has_mob_effect.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.filters.has_mob_effect",
  "type": "object",
  "title": "Has Mob Effect",
  "description": "Tests whether the Subject has the specified mob effect.",
  "required": ["value"],
  "properties": {
    "test": {
      "type": "string",
      "title": "Test Property",
      "description": "Tests whether the Subject has the specified mob effect."
    },
    "operator": {
      "$ref": "./types/operator.json"
    },
    "subject": {
      "$ref": "./types/subject.json"
    },
    "value": {
      "type": "string",
      "description": "The specified mob effect.",
      "title": "Value"
    }
  },
  "examples": [
    {
      "test": "has_mob_effect",
      "value": "bad_omen"
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HasMobEffect {}
