/* Raw contents of annotation.open_door.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.annotation.open_door",
  "additionalProperties": false,
  "description": "Allows the actor to open doors assuming that that flags set up for the component to use in navigation.",
  "type": "object",
  "title": "Annotation.open Door",
  "required": [],
  "properties": {}
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Annotation.openDoor {}
