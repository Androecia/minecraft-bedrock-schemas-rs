/* Raw contents of managed_wandering_trader.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.managed_wandering_trader",
  "type": "object",
  "title": "Managed Wandering Trader",
  "additionalProperties": false,
  "required": [],
  "properties": {},
  "description": "This component is used to implement part of the Wandering Trader behavior."
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ManagedWanderingTrader {}
