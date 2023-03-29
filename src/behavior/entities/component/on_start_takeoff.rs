/* Raw contents of on_start_takeoff.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.on_start_takeoff",
  "description": "Only usable by the Ender Dragon. Adds a trigger to call when this entity starts flying.",
  "title": "On Start Takeoff",
  "$ref": "../types/trigger.json"
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OnStartTakeoff {}
