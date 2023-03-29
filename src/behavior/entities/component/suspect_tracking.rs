/* Raw contents of suspect_tracking.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.suspect_tracking",
  "type": "object",
  "title": "Suspect Tracking",
  "additionalProperties": false,
  "required": [],
  "properties": {},
  "description": "Allows this entity to remember suspicious locations."
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SuspectTracking {}
