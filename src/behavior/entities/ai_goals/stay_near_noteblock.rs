/* Raw contents of stay_near_noteblock.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.stay_near_noteblock",
  "type": "object",
  "title": "Stay Near Noteblock",
  "description": "[EXPERIMENTAL BEHAVIOR] The entity will attempt to toss the items from its inventory to a nearby recently played noteblock.",
  "additionalProperties": false,
  "properties": {
    "priority": {
      "$ref": "types/priority.json"
    },
    "listen_time": {
      "title": "Listen Time",
      "type": "integer",
      "default": 0.0,
      "description": "Sets the time an entity should stay near a noteblock after hearing it."
    },
    "speed": {
      "title": "Speed",
      "type": "number",
      "default": 1.0,
      "description": "Sets the entity's speed when moving toward the block."
    },
    "start_distance": {
      "title": "Start Distance",
      "type": "number",
      "default": 10.0,
      "description": "Sets the distance the entity needs to be away from the block to attempt to start the goal."
    },
    "stop_distance": {
      "title": "Stop Distance",
      "type": "number",
      "default": 2.0,
      "description": "Sets the distance from the block the entity will attempt to reach."
    }
  }
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StayNearNoteblock {}
