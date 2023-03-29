/* Raw contents of block_climber.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.block_climber",
  "title": "Block Climber",
  "type": "object",
  "additionalProperties": false,
  "description": "Allows the player to detect and manuever on the scaffolding block.",
  "properties": {}
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockClimber {}
