/* Raw contents of loot.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.loot",
  "type": "object",
  "title": "Loot",
  "description": "sets the loot table for what items this entity drops upon death.",
  "additionalProperties": false,
  "required": ["table"],
  "examples": [{ "table": "loot_tables/empty.json" }],
  "properties": {
    "table": {
      "type": "string",
      "pattern": "^loot_tables/.*.json$",
      "description": "The path to the loot table, relative to the Behavior Pack's root.",
      "title": "Table",
      "default": "loot_tables/empty.json"
    }
  }
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Loot {}
