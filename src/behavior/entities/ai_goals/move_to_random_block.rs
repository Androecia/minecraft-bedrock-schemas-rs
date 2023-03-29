/* Raw contents of move_to_random_block.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.move_to_random_block",
  "type": "object",
  "title": "Move To Random Block",
  "description": "Allows mob to move towards a random block.",
  "additionalProperties": false,
  "required": [],
  "properties": {
    "priority": {
      "$ref": "./types/priority.json"
    },
    "speed_multiplier": {
      "$ref": "types/speed_multiplier.json"
    },
    "block_distance": {
      "type": "number",
      "default": 16,
      "description": "Defines the distance from the mob, in blocks, that the block to move to will be chosen.",
      "title": "Block Distance"
    },
    "within_radius": {
      "type": "number",
      "default": 0,
      "description": "Defines the distance in blocks the mob has to be from the block for the movement to be finished.",
      "title": "Within Radius"
    }
  },
  "examples": [
    {
      "block_distance": 16,
      "within_radius": 0
    }
  ]
}*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MoveToRandomBlock {}
