/* Raw contents of guardian_attack.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.guardian_attack",
  "type": "object",
  "title": "Guardian Attack",
  "description": "Allows this entity to use a laser beam attack. Can only be used by Guardians and Elder Guardians.",
  "additionalProperties": false,
  "required": [],
  "properties": {
    "priority": {
      "$ref": "./types/priority.json"
    },
    "elder_extra_magic_damage": {
      "title": "Elder Extra Magic Damage",
      "type": "integer",
      "default": 2,
      "description": "Amount of additional damage dealt from an elder guardian's magic attack."
    },
    "hard_mode_extra_magic_damage": {
      "title": "Hard Mode Extra Magic Damage",
      "type": "integer",
      "default": 2,
      "description": "In hard difficulty, amount of additional damage dealt from a guardian's magic attack."
    },
    "magic_damage": {
      "title": "Magic Damage",
      "type": "integer",
      "default": 1,
      "description": "Amount of damage dealt from a guardian's magic attack. Magic attack damage is added to the guardian's base attack damage."
    },
    "min_distance": {
      "title": "Min Distance",
      "type": "number",
      "default": 3,
      "description": "Guardian attack behavior stops if the target is closer than this distance (doesn't apply to elders)."
    },
    "sound_delay_time": {
      "title": "Sound Delay Time",
      "type": "number",
      "default": 0.5,
      "description": "Time (in seconds) to wait after starting an attack before playing the guardian attack sound."
    },
    "x_max_rotation": {
      "title": "X Max Rotation",
      "type": "number",
      "default": 90,
      "description": "Maximum rotation (in degrees), on the X-axis, this entity can rotate while trying to look at the target."
    },
    "y_max_head_rotation": {
      "title": "Y Max Head Rotation",
      "type": "number",
      "default": 90,
      "description": "Maximum rotation (in degrees), on the Y-axis, this entity can rotate its head while trying to look at the target."
    }
  }
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GuardianAttack {}
