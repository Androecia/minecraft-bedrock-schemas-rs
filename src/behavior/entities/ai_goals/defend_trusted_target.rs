/* Raw contents of defend_trusted_target.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.defend_trusted_target",
  "type": "object",
  "title": "Defend Trusted Target",
  "description": "Allows the mob to target another mob that hurts an entity it trusts.",
  "additionalProperties": false,
  "properties": {
    "priority": {
      "$ref": "types/priority.json"
    },
    "aggro_sound": {
      "title": "Aggro Sound",
      "$ref": "../../../../general/sound_event.json",
      "default": "",
      "description": "Sound to occasionally play while defending."
    },
    "attack_interval": {
      "title": "Attack Interval",
      "type": "integer",
      "default": 0,
      "description": "Time in seconds between attacks."
    },
    "must_see": {
      "title": "Must See",
      "type": "boolean",
      "default": false,
      "description": "If true, only entities in this mob's viewing range can be selected as targets."
    },
    "must_see_forget_duration": {
      "title": "Must See Forget Duration",
      "type": "number",
      "default": 3,
      "description": "Determines the amount of time in seconds that this mob will look for a target before forgetting about it and looking for a new one when the target isn't visible any more."
    },
    "on_defend_start": {
      "title": "On Defend Start",
      "$ref": "../types/event.json",
      "description": "The event to run when this mob starts to defend the entity it trusts."
    },
    "within_radius": {
      "title": "Within Radius",
      "type": "number",
      "default": 0,
      "description": "Distance in blocks that the target can be within to launch an attack."
    },
    "entity_types": {
      "$ref": "../types/entity_types.json",
      "description": "List of entity types this mob will startle (cause to jump) when it sneezes.",
      "title": "Entity Types"
    },
    "sound_chance": {
      "title": "Sound Chance",
      "type": "number",
      "default": 0.05,
      "description": "Probability that a sound will play.",
      "minimum": 0,
      "maximum": 1
    }
  },
  "examples": [
    {
      "aggro_sound": "",
      "attack_interval": 0,
      "must_see": false,
      "must_see_forget_duration": 3,
      "within_radius": 0,
      "entity_types": {},
      "sound_chance": 0.05
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DefendTrustedTarget {}
