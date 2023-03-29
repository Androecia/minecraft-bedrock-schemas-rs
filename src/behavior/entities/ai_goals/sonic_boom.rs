/* Raw contents of sonic_boom.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.sonic_boom",
  "additionalProperties": false,
  "type": "object",
  "title": "Sonic Boom",
  "description": "[EXPERIMENTAL BEHAVIOR] Plays the provided sounds and activates the `SONIC BOOM` actor flag during the specified duration",
  "properties": {
    "priority": { "$ref": "./types/priority.json" },
    "speed_multiplier": { "$ref": "types/speed_multiplier.json" },
    "attack_cooldown": {
      "title": "Attack Cooldown",
      "type": "number",
      "default": 5.0,
      "description": "Cooldown in seconds required after using this attack until the entity can use sonic boom again."
    },
    "attack_damage": {
      "title": "Attack Damage",
      "type": "number",
      "default": 30.0,
      "description": "Attack damage of the sonic boom."
    },
    "attack_range_horizontal": {
      "title": "Attack Range Horizontal",
      "type": "number",
      "default": 15.0,
      "description": "Horizontal range (in blocks) at which the sonic boom can damage the target."
    },
    "attack_range_vertical": {
      "title": "Attack Range Vertical",
      "type": "number",
      "default": 20.0,
      "description": "Vertical range (in blocks) at which the sonic boom can damage the target."
    },
    "attack_sound": {
      "title": "Attack Sound",
      "$ref": "../../../../general/sound_event.json",
      "default": "",
      "description": "Sound event for the attack."
    },
    "charge_sound": {
      "title": "Charge Sound",
      "$ref": "../../../../general/sound_event.json",
      "default": "",
      "description": "Sound event for the charge up."
    },
    "duration": {
      "title": "Duration",
      "type": "number",
      "default": 3.0,
      "description": "Goal duration in seconds."
    },
    "duration_until_attack_sound": {
      "title": "Duration Until Attack Sound",
      "type": "number",
      "default": 1.7,
      "description": "Duration in seconds until the attack sound is played."
    },
    "knockback_height_cap": {
      "title": "Knockback Height Cap",
      "type": "number",
      "default": 0.0,
      "description": "Height cap of the attack knockback's vertical delta."
    },
    "knockback_horizontal_strength": {
      "title": "Knockback Horizontal Strength",
      "type": "number",
      "default": 0.0,
      "description": "Horizontal strength of the attack's knockback applied to the attack target."
    },
    "knockback_vertical_strength": {
      "title": "Knockback Vertical Strength",
      "type": "number",
      "default": 0.0,
      "description": "Vertical strength of the attack's knockback applied to the attack target."
    }
  }
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SonicBoom {}
