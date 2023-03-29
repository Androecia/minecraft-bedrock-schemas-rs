/* Raw contents of experience_reward.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.experience_reward",
  "type": "object",
  "title": "Experience Reward",
  "description": "Defines the amount of experience rewarded when the entity dies or is successfully bred.",
  "additionalProperties": false,
  "properties": {
    "on_bred": {
      "type": ["string", "number"],
      "default": 0,
      "description": "A molang expression defining the amount of experience rewarded when this entity is successfully bred. An array of expressions adds each expression's result together for a final total.",
      "title": "On Bred"
    },
    "on_death": {
      "type": ["string", "number"],
      "default": 0,
      "description": "A molang expression defining the amount of experience rewarded when this entity dies. An array of expressions adds each expression's result together for a final total.",
      "title": "On Death"
    }
  },
  "examples": [
    {
      "on_bred": 0,
      "on_death": 0
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExperienceReward {}
