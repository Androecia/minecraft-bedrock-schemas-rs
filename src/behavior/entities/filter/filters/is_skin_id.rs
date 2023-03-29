/* Raw contents of is_skin_id.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.filters.is_skin_id",
  "type": "object",
  "title": "Is Skin Id",
  "description": "Returns true if the subject entity is the skin id number provided.",
  "properties": {
    "test": {
      "type": "string",
      "title": "Test Property",
      "description": "The test property."
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
      "title": "Value"
    }
  },
  "examples": [
    {
      "test": "is_skin_id",
      "value": 0
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsSkinId {}
