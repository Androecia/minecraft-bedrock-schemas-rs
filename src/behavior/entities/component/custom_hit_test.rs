/* Raw contents of custom_hit_test.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.custom_hit_test",
  "type": "object",
  "title": "Custom Hit Test",
  "description": "List of hitboxes for melee and ranged hits against the entity.",
  "additionalProperties": false,
  "properties": {
    "hitboxes": {
      "type": "array",
      "title": "Hitboxes",
      "description": "Defines a hitbox size and pivot to test against.",
      "items": {
        "type": "object",
        "title": "Hitbox",
        "description": "Defines a hitbox size and pivot to test against.",
        "additionalProperties": false,
        "properties": {
          "width": {
            "type": "number",
            "title": "Width",
            "description": "Height of the hitbox in blocks. A negative value will be assumed to be 0."
          },
          "height": {
            "type": "number",
            "title": "Height",
            "description": "Width and Depth of the hitbox in blocks. A negative value will be assumed to be 0."
          },
          "pivot": {
            "type": "array",
            "title": "Pivot",
            "description": "The offset from the entity's anchor where the hitbox will spawn.",
            "items": [
              { "type": "number", "title": "X" },
              { "type": "number", "title": "Y" },
              { "type": "number", "title": "Z" }
            ]
          }
        }
      }
    }
  },
  "examples": [
    {
      "hitboxes": [
        {
          "width": 1,
          "height": 1,
          "pivot": [0, 1, 0]
        }
      ]
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomHitTest {}
