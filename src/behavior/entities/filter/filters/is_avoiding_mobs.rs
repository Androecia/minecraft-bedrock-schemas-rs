/* Raw contents of is_avoiding_mobs.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.filters.is_avoiding_mobs",
  "type": "object",
  "title": "Is Avoiding Mobs",
  "description": "Returns true if the subject entity is fleeing from other mobs.",
  "properties": {
    "test": {
      "type": "string",
      "title": "Test Property",
      "description": "Returns true if the subject entity is fleeing from other mobs."
    },
    "operator": {
      "$ref": "./types/operator.json"
    },
    "subject": {
      "$ref": "./types/subject.json"
    },
    "value": {
      "title": "Value",
      "description": "True or false.",
      "type": "boolean",
      "default": true
    }
  },
  "examples": [
    {
      "test": "example",
      "value": true
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsAvoidingMobs {}
