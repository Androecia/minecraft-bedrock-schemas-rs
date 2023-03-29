/* Raw contents of hourly_clock_time.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.filters.hourly_clock_time",
  "type": "object",
  "title": "Hourly Clock Time",
  "description": "Compares the current 24 hour time with an int value in the range[0, 24000].",
  "required": ["value"],
  "properties": {
    "test": {
      "type": "string",
      "title": "Test Property",
      "description": "Compares the current 24 hour time with an int value in the range[0, 24000].",
      "const": "hourly_clock_time"
    },
    "operator": {
      "$ref": "./types/operator.json"
    },
    "subject": {
      "$ref": "./types/subject.json"
    },
    "value": {
      "type": "integer",
      "description": "(Required) An integer value set between 0 and 24000.",
      "minimum": 0,
      "maximum": 24000,
      "title": "Value"
    }
  },
  "examples": [
    {
      "test": "hourly_clock_time",
      "value": 0
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HourlyClockTime {}
