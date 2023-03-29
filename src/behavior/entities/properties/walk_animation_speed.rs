/* Raw contents of walk_animation_speed.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.walk_animation_speed",
  "additionalProperties": false,
  "type": "object",
  "title": "Walk Animation Speed",
  "description": "Sets the speed multiplier for this entity's walk animation speed.",
  "required": [],
  "properties": {
    "value": {
      "type": "number",
      "default": 1,
      "description": "The higher the number, the faster the animation for walking plays. A value of 1.0 means normal speed, while 2.0 means twice as fast",
      "title": "Value"
    }
  },
  "examples": [{ "value": 1 }]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WalkAnimationSpeed {}
