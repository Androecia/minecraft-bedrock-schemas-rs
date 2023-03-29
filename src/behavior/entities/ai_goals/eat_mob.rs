/* Raw contents of eat_mob.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.eat_mob",
  "type": "object",
  "title": "Eat Mob",
  "description": "[EXPERIMENTAL BEHAVIOR] Allows the entity to eat a specified Mob.",
  "additionalProperties": false,
  "required": [],
  "properties": {
    "priority": { "$ref": "./types/priority.json" },
    "eat_animation_time": {
      "title": "Eat Animation Time",
      "type": "number",
      "default": 1000000,
      "description": "Sets the time in seconds the eat animation should play for."
    },
    "eat_mob_sound": {
      "title": "Eat Mob Sound",
      "$ref": "../../../../general/sound_event.json",
      "default": "",
      "description": "Sets the sound that should play when eating a mob."
    },
    "loot_table": {
      "title": "Loot Table",
      "type": "string",
      "default": "",
      "description": "The loot table for loot to be dropped when eating a mob."
    },
    "pull_in_force": {
      "title": "Pull In Force",
      "type": "number",
      "default": 1000000,
      "description": "Sets the force which the mob-to-be-eaten is pulled towards the eating mob."
    },
    "reach_mob_distance": {
      "title": "Reach Mob Distance",
      "type": "number",
      "default": 1000000,
      "description": "Sets the desired distance to be reached before eating the mob."
    },
    "run_speed": {
      "title": "Run Speed",
      "type": "number",
      "default": 1000000,
      "description": "Sets the entity's speed when running toward the target."
    }
  }
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EatMob {}
