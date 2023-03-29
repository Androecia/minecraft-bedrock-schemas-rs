/* Raw contents of in_nether.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.filters.in_nether",
  "type": "object",
  "title": "In Nether",
  "description": "Returns true when the subject entity is in Nether.",
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
      "description": "True or false.",
      "type": "boolean",
      "default": true,
      "title": "Value"
    }
  },
  "examples": [
    {
      "test": "in_nether",
      "value": true
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InNether {}
