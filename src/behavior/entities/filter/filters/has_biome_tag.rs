/* Raw contents of has_biome_tag.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.filters.has_biome_tag",
  "type": "object",
  "title": "Has Biome Tag",
  "description": "Tests whether the biome the subject is in has the specified tag.",
  "required": ["value"],
  "properties": {
    "test": {
      "type": "string",
      "title": "Test Property",
      "description": "Tests whether the biome the subject is in has the specified tag."
    },
    "operator": {
      "$ref": "./types/operator.json"
    },
    "subject": {
      "$ref": "./types/subject.json"
    },
    "value": {
      "description": "(Required) The tag to look for.",
      "type": "string",
      "title": "Value"
    }
  },
  "examples": [
    {
      "test": "has_biome_tag",
      "value": "monster"
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HasBiomeTag {}
