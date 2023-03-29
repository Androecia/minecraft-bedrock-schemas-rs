/* Raw contents of weather_at_position.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.filters.weather_at_position",
  "type": "object",
  "title": "Weather At Position",
  "description": "Tests the current weather, at the actor's position, against a provided weather value.",
  "required": ["value"],
  "properties": {
    "test": {
      "type": "string",
      "title": "Test",
      "description": "The test property."
    },
    "operator": {
      "$ref": "./types/operator.json"
    },
    "subject": {
      "$ref": "./types/subject.json"
    },
    "value": {
      "description": "The Family name to look for.",
      "type": "string",
      "title": "Value",
      "examples": ["thunderstorm"]
    }
  },
  "examples": [
    {
      "test": "weather_at_position",
      "value": "thunderstorm"
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WeatherAtPosition {}
