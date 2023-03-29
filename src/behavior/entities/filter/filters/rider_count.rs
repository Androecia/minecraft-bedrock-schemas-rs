/* Raw contents of rider_count.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.filters.rider_count",
  "type": "object",
  "title": "Rider Count",
  "description": "Returns the number of riders on this entity.",
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
      "description": "An integer value.",
      "type": "integer",
      "title": "Value"
    }
  },
  "examples": [
    {
      "test": "rider_count",
      "value": 0
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RiderCount {}
