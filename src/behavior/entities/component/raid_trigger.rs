/* Raw contents of raid_trigger.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.raid_trigger",
  "type": "object",
  "title": "Raid Trigger",
  "additionalProperties": false,
  "description": "Attempts to trigger a raid at the entity's location.",
  "required": [],
  "properties": {
    "triggered_event": {
      "$ref": "../types/event.json",
      "description": "Event to run we attempt to trigger a raid on the village.",
      "title": "Triggered Event"
    }
  }
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RaidTrigger {}
