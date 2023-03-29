/* Raw contents of move_to_poi.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.move_to_poi",
  "type": "object",
  "title": "Move To Poi",
  "additionalProperties": false,
  "description": "Allows the mob to move to a POI if able to.",
  "required": [],
  "properties": {
    "priority": {
      "$ref": "./types/priority.json"
    },
    "speed_multiplier": {
      "$ref": "./types/speed_multiplier.json"
    },
    "poi_type": {
      "type": "string",
      "description": "Tells the goal what POI type it should be looking for.",
      "title": "Point Of Interest Type",
      "enum": ["bed", "jobsite", "meeting_area"]
    }
  },
  "examples": [
    {
      "poi_type": "example"
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MoveToPoi {}
