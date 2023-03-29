/* Raw contents of group_size.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.group_size",
  "type": "object",
  "title": "Group Size",
  "description": "Keeps track of entity group size in the given radius.",
  "additionalProperties": false,
  "properties": {
    "filters": {
      "$ref": "../../filters/filters.json",
      "description": "The list of conditions that must be satisfied for other entities to be counted towards group size."
    },
    "radius": {
      "type": "number",
      "default": 16,
      "description": "Radius from center of entity.",
      "title": "Radius"
    }
  },
  "examples": [
    {
      "radius": 16
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupSize {}
