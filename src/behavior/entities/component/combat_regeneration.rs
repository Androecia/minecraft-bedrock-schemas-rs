/* Raw contents of combat_regeneration.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.combat_regeneration",
  "type": "object",
  "title": "Combat Regeneration",
  "description": "Gives Regeneration I and removes Mining Fatigue from the mob that kills the Actor`s attack target.",
  "additionalProperties": false,
  "properties": {
    "apply_to_family": {
      "type": "boolean",
      "default": false,
      "description": "Determines if the mob will grant mobs of the same type combat buffs if they kill the target.",
      "title": "Apply To Family"
    },
    "apply_to_self": {
      "type": "boolean",
      "default": false,
      "description": "Determines if the mob will grant itself the combat buffs if it kills the target.",
      "title": "Apply To Self"
    },
    "regeneration_duration": {
      "type": "integer",
      "default": 5,
      "description": "The duration in seconds of Regeneration I added to the mob.",
      "title": "Regeneration Duration"
    }
  },
  "examples": [
    {
      "apply_to_family": false,
      "apply_to_self": false,
      "regeneration_duration": 5
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CombatRegeneration {}
