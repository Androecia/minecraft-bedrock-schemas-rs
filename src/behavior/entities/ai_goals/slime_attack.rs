/* Raw contents of slime_attack.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.slime_attack",
  "additionalProperties": false,
  "description": "Can only be used by Slimes and Magma Cubes. Allows the mob to use a melee attack like the slime's.",
  "type": "object",
  "title": "Slime Attack",
  "properties": {
    "priority": { "$ref": "./types/priority.json" },
    "speed_multiplier": { "$ref": "types/speed_multiplier.json" },
    "set_persistent": {
      "title": "Set Persistent",
      "type": "boolean",
      "default": false,
      "description": "Allows the actor to be set to persist upon targeting a player."
    },
    "x_max_rotation": {
      "title": "X Max Rotation",
      "type": "number",
      "default": 10,
      "description": "Maximum rotation (in degrees), on the X-axis, this entity can rotate while trying to look at the target."
    },
    "y_max_rotation": {
      "title": "Y Max Rotation",
      "type": "number",
      "default": 10,
      "description": "Maximum rotation (in degrees), on the Y-axis, this entity can rotate while trying to look at the target."
    }
  },
  "examples": [
    {
      "set_persistent": false
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SlimeAttack {}
