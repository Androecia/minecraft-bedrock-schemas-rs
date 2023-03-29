/* Raw contents of burns_in_daylight.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.burns_in_daylight",
  "title": "Burns In Daylight",
  "additionalProperties": false,
  "description": "Specifies if/how a mob burns in daylight.",
  "required": [],
  "properties": {},
  "oneOf": [
    {
      "type": "boolean"
    },
    {
      "type": "object"
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BurnsInDaylight {}
