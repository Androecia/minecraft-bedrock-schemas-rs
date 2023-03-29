/* Raw contents of leashable.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.leashable",
  "type": "object",
  "title": "Leashable",
  "description": "Allows this entity to be leashed and defines the conditions and events for this entity when is leashed.",
  "additionalProperties": false,
  "properties": {
    "can_be_stolen": {
      "type": "boolean",
      "default": false,
      "description": "If true, players can leash this entity even if it is already leashed to another mob.",
      "title": "Can Be Stolen"
    },
    "hard_distance": {
      "type": "number",
      "default": 6,
      "description": "Distance in blocks at which the leash stiffens, restricting movement.",
      "title": "Hard Distance"
    },
    "max_distance": {
      "type": "number",
      "default": 10,
      "description": "Distance in blocks at which the leash breaks.",
      "title": "Maximum Distance"
    },
    "on_leash": {
      "$ref": "../types/event_object.json",
      "description": "Event to call when this entity is leashed.",
      "title": "On Leash"
    },
    "on_unleash": {
      "$ref": "../types/event_object.json",
      "description": "Event to call when this entity is unleashed.",
      "title": "On Unleash"
    },
    "soft_distance": {
      "type": "number",
      "default": 4,
      "description": "Distance in blocks at which the `spring` effect starts acting to keep this entity close to the entity that leashed it.",
      "title": "Soft Distance"
    }
  },
  "examples": [
    {
      "can_be_stolen": false,
      "hard_distance": 6,
      "max_distance": 10,
      "soft_distance": 4
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Leashable {}
