/* Raw contents of game_event_movement_tracking.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.game_event_movement_tracking",
  "additionalProperties": false,
  "description": "Allows an entity to emit `entityMove`, `swim` and `flap` game events, depending on the block the entity is moving through. It is added by default to every mob. Add it again to override its behavior.",
  "type": "object",
  "title": "Game Event Movement Tracking",
  "properties": {
    "emit_flap": {
      "title": "emit flap",
      "type": "boolean",
      "default": false,
      "description": "If true, the `flap` game event will be emitted when the entity moves through air."
    },
    "emit_move": {
      "title": "emit move",
      "type": "boolean",
      "default": true,
      "description": "If true, the `entityMove` game event will be emitted when the entity moves on ground or through a solid."
    },
    "emit_swim": {
      "title": "emit swim",
      "type": "boolean",
      "default": true,
      "description": "If true, the `swim` game event will be emitted when the entity moves through a liquid."
    }
  }
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GameEventMovementTracking {}
