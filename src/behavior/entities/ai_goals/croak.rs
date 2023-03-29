/* Raw contents of croak.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.croak",
  "type": "object",
  "title": "Croak",
  "description": "[EXPERIMENTAL BEHAVIOR] Allows the entity to croak at a random time interval with configurable conditions.",
  "additionalProperties": false,
  "required": [],
  "properties": {
    "priority": { "$ref": "./types/priority.json" },
    "duration": {
      "title": "Duration",
      "description": "Random range in seconds after which the croaking stops. Can also be a constant.",
      "$ref": "../../../../general/vectors/integer2OrValue.json"
    },
    "filters": {
      "title": "Filters",
      "$ref": "../../filters/filters.json",
      "description": "Conditions for the behavior to start and keep running. The interval between runs only starts after passing the filters."
    },
    "interval": {
      "title": "Interval",
      "description": "Random range in seconds between runs of this behavior. Can also be a constant.",
      "$ref": "../../../../general/vectors/integer2OrValue.json"
    }
  }
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Croak {}
