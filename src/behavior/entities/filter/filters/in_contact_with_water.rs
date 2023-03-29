/* Raw contents of in_contact_with_water.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.filters.in_contact_with_water",
  "type": "object",
  "title": "In Contact With Water",
  "description": "Returns true when the subject entity in contact with any water: water, rain, splash water bottle.",
  "properties": {
    "test": {
      "type": "string",
      "title": "Test Property",
      "description": "Returns true when the subject entity in contact with any water: water, rain, splash water bottle."
    },
    "operator": {
      "$ref": "./types/operator.json"
    },
    "subject": {
      "$ref": "./types/subject.json"
    },
    "value": {
      "title": "Value",
      "description": "(Optional) true or false.",
      "type": "boolean",
      "default": true
    }
  },
  "examples": [
    {
      "test": "in_contact_with_water",
      "value": true
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InContactWithWater {}
