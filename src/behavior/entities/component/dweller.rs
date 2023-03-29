/* Raw contents of dweller.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.dweller",
  "type": "object",
  "title": "Dweller",
  "description": "Allows a mob to join and migrate between villages and other dwellings.",
  "additionalProperties": false,
  "properties": {
    "dwelling_type": {
      "type": "string",
      "title": "Dwelling Type",
      "description": "The type of dwelling the mob wishes to join. Current Types: village",
      "enum": ["village"]
    },
    "dweller_role": {
      "type": "string",
      "title": "Dwelling Role",
      "description": "The role of which the mob plays in the dwelling. Current Roles: inhabitant, defender, hostile, passive.",
      "enum": ["inhabitant", "defender", "hostile", "passive"]
    },
    "update_interval_base": {
      "type": "number",
      "title": "Update Interval Base",
      "description": "How often the mob checks on their dwelling status in ticks. Positive values only.",
      "minimum": 0
    },
    "update_interval_variant": {
      "type": "number",
      "title": "Update Interval Variant",
      "description": "The variant value in ticks that will be added to the update_interval_base."
    },
    "can_find_poi": {
      "type": "boolean",
      "title": "Can Find Poi",
      "description": "Whether or not the mob can find and add POI's to the dwelling."
    },
    "first_founding_reward": {
      "type": "integer",
      "title": "First Founding Reward",
      "description": "How much reputation should the players be rewarded on first founding?."
    },
    "can_migrate": {
      "type": "boolean",
      "title": "Can Migrate",
      "description": "Can this mob migrate between dwellings? Or does it only have its initial dwelling?."
    },
    "dwelling_bounds_tolerance": {
      "title": "Dwelling Bounds Tolerance",
      "type": "number",
      "description": "A padding distance for checking if the mob is within the dwelling."
    },
    "preferred_profession": {
      "type": "string",
      "title": "Preferred Profession",
      "description": "Allows the user to define a starting profession for this particular Dweller, instead of letting them choose organically. (They still need to gain experience from trading before this takes effect.)"
    }
  },
  "examples": [
    {
      "dwelling_type": "village",
      "dweller_role": "inhabitant",
      "update_interval_base": 0,
      "update_interval_variant": 0,
      "preferred_profession": "example",
      "can_find_poi": true,
      "can_migrate": true,
      "first_founding_reward": 0
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Dweller {}
