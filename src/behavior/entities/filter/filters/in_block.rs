/* Raw contents of in_block.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.filters.in_block",
  "type": "object",
  "title": "In Block",
  "description": "Returns true when the subject entity is inside a specified Block type.",
  "properties": {
    "test": {
      "type": "string",
      "title": "Test Property",
      "description": "Returns true when the subject entity is inside a specified Block type."
    },
    "operator": {
      "$ref": "./types/operator.json",
      "description": "(Optional) The comparison to apply with `value`.",
      "default": "equals",
      "title": "Operator"
    },
    "subject": {
      "$ref": "./types/subject.json",
      "description": "(Optional) The subject of this filter test.",
      "default": "self",
      "title": "Subject"
    },
    "value": {
      "type": "string",
      "description": "(Optional) A string value.",
      "title": "Value"
    }
  },
  "examples": [
    {
      "test": "in_block",
      "value": "minecraft:water"
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InBlock {}
