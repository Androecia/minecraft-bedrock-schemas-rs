/* Raw contents of float_wander.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.float_wander",
  "type": "object",
  "title": "Float Wander",
  "description": "Allows the mob to float around like the Ghast.",
  "additionalProperties": false,
  "required": [],
  "properties": {
    "priority": {
      "$ref": "./types/priority.json"
    },
    "xz_dist": {
      "type": "integer",
      "default": 10,
      "description": "Distance in blocks on ground that the mob will look for a new spot to move to. Must be at least 1",
      "title": "Xz Dist"
    },
    "y_dist": {
      "type": "integer",
      "default": 7,
      "description": "Distance in blocks that the mob will look up or down for a new spot to move to. Must be at least 1",
      "title": "Y Dist"
    },
    "y_offset": {
      "type": "number",
      "default": 0.0,
      "description": "Height in blocks to add to the selected target position.",
      "title": "Y Offset"
    },
    "must_reach": {
      "type": "boolean",
      "default": false,
      "description": "If true, the point has to be reachable to be a valid target.",
      "title": "Must Reach"
    },
    "random_reselect": {
      "type": "boolean",
      "default": false,
      "description": "If true, the mob will randomly pick a new point while moving to the previously selected one.",
      "title": "Random Reselect"
    },
    "float_duration": {
      "$ref": "../types/range_number_type.json",
      "default": [0.0, 0.0],
      "description": "Range of time in seconds the mob will float around before landing and choosing to do something else.",
      "title": "Float Duration"
    }
  },
  "examples": [
    {
      "xz_dist": 10,
      "y_dist": 7,
      "y_offset": 0.0,
      "must_reach": false,
      "random_reselect": false
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FloatWander {}
