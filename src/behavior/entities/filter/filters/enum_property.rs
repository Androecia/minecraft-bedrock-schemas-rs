/* Raw contents of enum_property.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.filters.enum_property",
  "type": "object",
  "title": "Enum Property",
  "description": "Returns true when the enum actor property matches the value provided.",
  "required": ["value", "domain"],
  "examples": [{ "test": "bool_property", "value": true }],
  "properties": {
    "test": {
      "type": "string",
      "title": "Test Property",
      "description": "Returns true when the enum actor property matches the value provided."
    },
    "domain": {
      "description": "(Required) The property name to look for",
      "title": "Operator",
      "type": "string"
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
      "title": "Value",
      "description": "(Required) A string value."
    }
  }
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnumProperty {}
