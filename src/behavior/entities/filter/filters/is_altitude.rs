/* Raw contents of is_altitude.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.filters.is_altitude",
  "type": "object",
  "title": "Is Altitude",
  "description": "Tests the current altitude against a provided value. 0= bedrock elevation.",
  "required": ["value"],
  "properties": {
    "test": {
      "type": "string",
      "title": "Test Property",
      "description": "Tests the current altitude against a provided value. 0= bedrock elevation."
    },
    "operator": {
      "$ref": "./types/operator.json"
    },
    "subject": {
      "$ref": "./types/subject.json"
    },
    "value": {
      "type": "integer",
      "description": "The altitude value to compare with.",
      "minimum": 0,
      "title": "Value"
    }
  },
  "examples": [
    {
      "test": "example",
      "value": 0
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsAltitude {}
