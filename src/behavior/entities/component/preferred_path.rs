/* Raw contents of preferred_path.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.preferred_path",
  "type": "object",
  "title": "Preferred Path",
  "description": "Specifies costing information for mobs that prefer to walk on preferred paths.",
  "additionalProperties": false,
  "properties": {
    "default_block_cost": {
      "type": "number",
      "default": 0,
      "description": "Cost for non-preferred blocks.",
      "title": "Default Block Cost"
    },
    "jump_cost": {
      "type": "integer",
      "default": 0,
      "description": "Added cost for jumping up a node.",
      "title": "Jump Cost"
    },
    "max_fall_blocks": {
      "type": "integer",
      "default": 3,
      "description": "Distance mob can fall without taking damage.",
      "title": "Maximum Fall Blocks"
    },
    "preferred_path_blocks": {
      "type": "array",
      "title": "Preferred Path Blocks",
      "description": "A list of blocks with their associated cost.",
      "items": {
        "additionalProperties": false,
        "type": "object",
        "description": "Blocks cost.",
        "properties": {
          "cost": {
            "type": "number"
          },
          "blocks": {
            "type": "array",
            "items": {
              "$ref": "../../../../general/block/reference.json"
            }
          }
        }
      }
    }
  },
  "examples": [
    {
      "default_block_cost": 0,
      "jump_cost": 0,
      "max_fall_blocks": 3,
      "preferred_path_blocks": []
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PreferredPath {}
