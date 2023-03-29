/* Raw contents of navigation.swim.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.navigation.swim",
  "title": "Navigation Swim",
  "description": "Allows this entity to generate paths that include water.",
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "avoid_damage_blocks": {
      "type": "boolean",
      "default": false,
      "description": "Tells the pathfinder to avoid blocks that cause damage when finding a path.",
      "title": "Avoid Damage Blocks"
    },
    "avoid_portals": {
      "type": "boolean",
      "default": false,
      "description": "Tells the pathfinder to avoid portals (like nether portals) when finding a path.",
      "title": "Avoid Portals"
    },
    "avoid_sun": {
      "type": "boolean",
      "default": false,
      "description": "Whether or not the pathfinder should avoid tiles that are exposed to the sun when creating paths.",
      "title": "Avoid Sun"
    },
    "avoid_water": {
      "type": "boolean",
      "default": false,
      "description": "Tells the pathfinder to avoid water when creating a path.",
      "title": "Avoid Water"
    },
    "blocks_to_avoid": {
      "type": "array",
      "description": "Tells the pathfinder which blocks to avoid when creating a path.",
      "title": "Blocks To Avoid",
      "items": {
        "title": "Block",
        "description": "Tells the pathfinder which blocks to avoid when creating a path.",
        "$ref": "../../../../general/block/reference.json"
      }
    },
    "can_breach": {
      "type": "boolean",
      "default": false,
      "description": "Tells the pathfinder whether or not it can jump out of water (like a dolphin).",
      "title": "Can Breach"
    },
    "can_break_doors": {
      "type": "boolean",
      "default": false,
      "description": "Tells the pathfinder that it can path through a closed door and break it.",
      "title": "Can Break Doors"
    },
    "can_jump": {
      "type": "boolean",
      "default": true,
      "description": "Tells the pathfinder whether or not it can jump up blocks.",
      "title": "Can Jump"
    },
    "can_open_doors": {
      "type": "boolean",
      "default": false,
      "description": "Tells the pathfinder that it can path through a closed door assuming the AI will open the door.",
      "title": "Can Open Doors"
    },
    "can_open_iron_doors": {
      "type": "boolean",
      "default": false,
      "description": "Tells the pathfinder that it can path through a closed iron door assuming the AI will open the door.",
      "title": "Can Open Iron Doors"
    },
    "can_pass_doors": {
      "type": "boolean",
      "default": true,
      "description": "Whether a path can be created through a door.",
      "title": "Can Pass Doors"
    },
    "can_path_from_air": {
      "type": "boolean",
      "default": false,
      "description": "Tells the pathfinder that it can start pathing when in the air.",
      "title": "Can Path From Air"
    },
    "can_path_over_lava": {
      "type": "boolean",
      "default": false,
      "description": "Tells the pathfinder whether or not it can travel on the surface of the lava.",
      "title": "Can Path Over Lava"
    },
    "can_path_over_water": {
      "type": "boolean",
      "default": false,
      "description": "Tells the pathfinder whether or not it can travel on the surface of the water.",
      "title": "Can Path Over Water"
    },
    "can_sink": {
      "type": "boolean",
      "default": true,
      "description": "Tells the pathfinder whether or not it will be pulled down by gravity while in water.",
      "title": "Can Sink"
    },
    "can_swim": {
      "type": "boolean",
      "default": false,
      "description": "Tells the pathfinder whether or not it can path anywhere through water and plays swimming animation along that path.",
      "title": "Can Swim"
    },
    "can_walk": {
      "type": "boolean",
      "default": true,
      "description": "Tells the pathfinder whether or not it can walk on the ground outside water.",
      "title": "Can Walk"
    },
    "can_walk_in_lava": {
      "type": "boolean",
      "default": false,
      "description": "Tells the pathfinder whether or not it can travel in lava like walking on ground.",
      "title": "Can Walk In Lava"
    },
    "is_amphibious": {
      "type": "boolean",
      "default": false,
      "description": "Tells the pathfinder whether or not it can walk on the ground underwater.",
      "title": "Is Amphibious"
    }
  },
  "examples": [
    {},
    {
      "avoid_damage_blocks": true,
      "can_pass_doors": true,
      "can_jump": true
    },
    {
      "avoid_damage_blocks": false,
      "avoid_portals": false,
      "avoid_sun": false,
      "avoid_water": false,
      "blocks_to_avoid": [],
      "can_breach": false,
      "can_break_doors": false,
      "can_jump": true,
      "can_open_doors": false,
      "can_open_iron_doors": false,
      "can_pass_doors": true,
      "can_path_from_air": false,
      "can_path_over_lava": false,
      "can_path_over_water": false,
      "can_sink": true,
      "can_swim": false,
      "can_walk": true,
      "can_walk_in_lava": false,
      "is_amphibious": false
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Navigation.swim {}
