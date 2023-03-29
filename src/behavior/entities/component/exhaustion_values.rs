/* Raw contents of exhaustion_values.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.exhaustion_values",
  "type": "object",
  "title": "Exhaustion Values",
  "additionalProperties": false,
  "description": "Defines how much exhaustion each player action should take.",
  "properties": {
    "attack": {
      "title": "Attack",
      "type": "number",
      "default": 0.1,
      "description": "Amount of exhaustion applied when attacking."
    },
    "damage": {
      "title": "Damage",
      "type": "number",
      "default": 0.1,
      "description": "Amount of exhaustion applied when taking damage."
    },
    "heal": {
      "title": "Heal",
      "type": "number",
      "default": 6,
      "description": "Amount of exhaustion applied when healed through food regeneration."
    },
    "jump": {
      "title": "Jump",
      "type": "number",
      "default": 0.05,
      "description": "Amount of exhaustion applied when jumping."
    },
    "mine": {
      "title": "Mine",
      "type": "number",
      "default": 0.005,
      "description": "Amount of exhaustion applied when mining."
    },
    "sprint": {
      "title": "Sprint",
      "type": "number",
      "default": 0.01,
      "description": "Amount of exhaustion applied when sprinting."
    },
    "sprint_jump": {
      "title": "Sprint Jump",
      "type": "number",
      "default": 0.2,
      "description": "Amount of exhaustion applied when sprint jumping."
    },
    "swim": {
      "title": "Swim",
      "type": "number",
      "default": 0.01,
      "description": "Amount of exhaustion applied when swimming."
    },
    "walk": {
      "title": "Walk",
      "type": "number",
      "default": 0,
      "description": "Amount of exhaustion applied when walking."
    }
  },
  "examples": [
    {
      "heal": 6,
      "jump": 0.05,
      "sprint_jump": 0.2,
      "mine": 0.005,
      "attack": 0.1,
      "damage": 0.1,
      "walk": 0.0,
      "sprint": 0.01,
      "swim": 0.01
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExhaustionValues {}
