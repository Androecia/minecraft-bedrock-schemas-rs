/* Raw contents of inactivity_timer.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.filters.inactivity_timer",
  "type": "object",
  "title": "Inactivity Timer",
  "description": "Tests if the specified duration in seconds of inactivity for despawning has been reached.",
  "required": ["value"],
  "properties": {
    "test": {
      "type": "string",
      "title": "Test",
      "description": "The test property."
    },
    "operator": {
      "$ref": "./types/operator.json"
    },
    "subject": {
      "$ref": "./types/subject.json"
    },
    "value": {
      "description": "The Family name to look for.",
      "type": "integer",
      "title": "Value"
    }
  },
  "examples": [
    {
      "test": "inactivity_timer",
      "value": 0
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InactivityTimer {}
