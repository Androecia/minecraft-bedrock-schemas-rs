/* Raw contents of sittable.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.sittable",
  "description": "Defines the entity's `sit` state.",
  "type": "object",
  "title": "Sittable",
  "additionalProperties": false,
  "required": [],
  "properties": {
    "sit_event": {
      "$ref": "../types/event_object.json",
      "description": "Event to run when the entity enters the `sit` state.",
      "title": "Sit Event"
    },
    "stand_event": {
      "$ref": "../types/event_object.json",
      "description": "Event to run when the entity exits the `sit` state.",
      "title": "Stand Event"
    }
  }
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sittable {}
