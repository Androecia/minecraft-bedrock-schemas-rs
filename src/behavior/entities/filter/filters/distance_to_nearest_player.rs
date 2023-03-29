/* Raw contents of distance_to_nearest_player.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.filters.distance_to_nearest_player",
  "type": "object",
  "title": "Distance To Nearest Player",
  "description": "Compares the distance to the nearest Player with a float value.",
  "required": ["value"],
  "examples": [{ "test": "distance_to_nearest_player", "value": 5.7 }],
  "properties": {
    "test": { "type": "string", "title": "Test Property", "description": "Compares the distance to the nearest Player with a float value." },
    "operator": { "$ref": "./types/operator.json" },
    "subject": { "$ref": "./types/subject.json" },
    "value": { "type": "number", "description": "(Required) A floating point value.", "title": "Value" }
  }
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DistanceToNearestPlayer {}
