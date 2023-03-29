/* Raw contents of tick_world.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.tick_world",
  "description": "Defines if the entity ticks the world and the radius around it to tick.",
  "type": "object",
  "title": "Tick World",
  "additionalProperties": false,
  "required": [],
  "properties": {
    "distance_to_players": {
      "type": "number",
      "default": 128,
      "description": "The distance at which the closest player has to be before this entity despawns. This option will be ignored if never_despawn is true. Min: 128 blocks.",
      "minimum": 128,
      "title": "Distance To Players"
    },
    "never_despawn": {
      "type": "boolean",
      "default": true,
      "description": "If true, this entity will not despawn even if players are far away. If false, distance_to_players will be used to determine when to despawn.",
      "title": "Never Despawn"
    },
    "radius": {
      "type": "integer",
      "default": 2,
      "description": "The area around the entity to tick. Default: 2. Allowed range: 2-6.",
      "minimum": 2,
      "maximum": 6,
      "title": "Radius"
    }
  },
  "examples": [
    {
      "distance_to_players": 128,
      "never_despawn": true,
      "radius": 2
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TickWorld {}
