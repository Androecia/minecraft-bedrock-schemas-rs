/* Raw contents of wither_random_attack_pos_goal.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.wither_random_attack_pos_goal",
  "description": "Allows the wither to launch random attacks. Can only be used by the Wither Boss.",
  "additionalProperties": false,
  "type": "object",
  "title": "Wither Random Attack Pos Goal",
  "required": [],
  "properties": {
    "priority": {
      "$ref": "./types/priority.json"
    }
  }
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WitherRandomAttackPosGoal {}
