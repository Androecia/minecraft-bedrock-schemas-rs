/* Raw contents of scaffolding_climber.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.scaffolding_climber",
  "$comment": "DEPRECATED"
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScaffoldingClimber {}
