/* Raw contents of boss.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.boss",
  "type": "object",
  "title": "Boss",
  "additionalProperties": false,
  "description": "The current state of the boss for updating the boss HUD.",
  "required": [],
  "properties": {
    "hud_range": {
      "type": "integer",
      "default": 55,
      "description": "The Maximum distance from the boss at which the boss's health bar is present on the players screen.",
      "title": "Hud Range"
    },
    "name": {
      "type": "string",
      "default": "",
      "description": "The name that will be displayed above the boss's health bar.",
      "title": "Name"
    },
    "should_darken_sky": {
      "type": "boolean",
      "default": false,
      "description": "Whether the sky should darken in the presence of the boss.",
      "title": "Should Darken Sky"
    }
  },
  "examples": [
    {
      "hud_range": 55,
      "name": "",
      "should_darken_sky": false
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Boss {}
