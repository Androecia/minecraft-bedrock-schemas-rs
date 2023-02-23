




/*{
    "$schema": "http://json-schema.org/draft-07/schema",
    "$id": "blockception.minecraft.behavior.dialogue",
    "examples": [
        {
            "format_version": "1.19.0",
            "minecraft:npc_dialogue": {
                "scenes": [
                    {
                        "scene_tag": "fast_travel",
                        "npc_name": { "rawtext": [{ "translate": "dialogue.guide.name" }] },
                        "text": { "rawtext": [{ "translate": "dialogue.fast_travel.body", "with": ["\n"] }] },
                        "buttons": []
                    }
                ]
            }
        }
    ],
    "type": "object",
    "title": "NPC Dialogue",
    "description": "Specifies the dialogue scenes.",
    "$comment": "UNDOCUMENTED",
    "additionalProperties": false,
    "required": ["format_version", "minecraft:npc_dialogue"],
    "properties": {
        "format_version": { "$ref": "../../general/format_version.json" },
        "minecraft:npc_dialogue": {
            "title": "NPC Dialogue",
            "description": "Specifies the dialogue of an npc.",
            "$comment": "UNDOCUMENTED",
            "type": "object",
            "additionalProperties": false,
            "properties": {
                "scenes": {
                    "title": "Scenes",
                    "description": "The different scenes.",
                    "$comment": "UNDOCUMENTED",
                    "type": "array",
                    "minItems": 1,
                    "items": {
                        "title": "Scene",
                        "description": "A single scene specification.",
                        "$comment": "UNDOCUMENTED",
                        "type": "object",
                        "additionalProperties": false,
                        "required": ["scene_tag"],
                        "examples": [{ "scene_tag": "foo_intro" }],
                        "properties": {
                            "buttons": {
                                "title": "Buttons",
                                "description": "This is where you can create buttons for your NPC.",
                                "type": "array",
                                "maxItems": 6,
                                "items": {
                                    "title": "Button",
                                    "description": "This is where you can create buttons for your NPC.",
                                    "type": "object",
                                    "additionalProperties": false,
                                    "properties": {
                                        "name": {
                                            "title": "Name",
                                            "description": "Set the text that is going to be displayed on your NPC’s button.",
                                            "oneOf": [{ "type": "string" }, { "$ref": "../../general/rawtext/rawtext.json" }]
                                        },
                                        "commands": {
                                            "type": "array",
                                            "description": "allows you to add commands which will be run in-game when the button is pressed.",
                                            "title": "Commands",
                                            "items": {
                                                "title": "Minecraft Command",
                                                "description": "The commands to execute.",
                                                "type": "string",
                                                "pattern": "^/[a-z].*$"
                                            }
                                        }
                                    }
                                }
                            },
                            "npc_name": {
                                "title": "NPC Name",
                                "description": "This is where you can add or change a name for your NPC dialogue box. This is an optional property that is useful for dynamically changing NPC names.",
                                "oneOf": [{ "type": "string" }, { "$ref": "../../general/rawtext/rawtext.json" }]
                            },
                            "on_close_commands": {
                                "type": "array",
                                "description": "This is where you can define which commands will fire when the NPC dialogue box closes.",
                                "title": "On Close Commands",
                                "items": {
                                    "title": "Minecraft Command",
                                    "description": "A minecraft command to execute.",
                                    "type": "string",
                                    "pattern": "^/[a-z].*$"
                                }
                            },
                            "on_open_commands": {
                                "type": "array",
                                "description": "This is where you can define which commands will fire when the NPC dialogue box opens.",
                                "title": "On Close Commands",
                                "items": {
                                    "title": "Minecraft Command",
                                    "description": "A minecraft command to execute.",
                                    "type": "string",
                                    "pattern": "^/[a-z].*$"
                                }
                            },
                            "scene_tag": {
                                "title": "Scene Tag",
                                "description": "This is the name you will use to call this scene in-game. This is a required property.",
                                "type": "string"
                            },
                            "text": {
                                "title": "Text",
                                "description": "This is where you enter the dialogue you want your NPC to display in-game for this scene. You can type the dialogue text directly here or use raw text if you are using a language file. This is an optional property, but without it your NPC dialogue box will be empty.",
                                "oneOf": [{ "type": "string" }, { "$ref": "../../general/rawtext/rawtext.json" }]
                            }
                        }
                    }
                }
            }
        }
    }
}
 */

use crate::general::version::Version;


use serde::{Deserialize, Serialize};

/// Specifies the dialogue of an npc.

#[derive( Clone,Serialize, Deserialize)]
struct  Dialogue {
    format_version: Version,

    /// The different scenes.
    #[serde(rename = "minecraft:npc_dialogue")]
    npc_dialogue: NpcDialogue,
}

#[derive( Clone, Serialize, Deserialize)]
/// Specifies the dialogue of an npc.
struct NpcDialogue {

    /// The different scenes.
    scenes: Vec<Scene>,
}


#[derive(Serialize, Deserialize,Clone)]
/// A single scene specification.
struct Scene {

        /// This is where you can create buttons for your NPC.
        buttons: Vec<Button>,

        /// This is where you can add or change a name for your NPC dialogue box. This is an optional property that is useful for dynamically changing NPC names.
        npc_name: Option<String>,

        /// This is where you can define which commands will fire when the NPC dialogue box closes.
        on_close_commands: Option<Vec<Command>>,

        /// This is where you can define which commands will fire when the NPC dialogue box opens.
        on_open_commands: Option<Vec<Command>>,

        /// This is the name you will use to call this scene in-game. This is a required property.
        scene_tag: String,

        /// This is where you enter the dialogue you want your NPC to display in-game for this scene. You can type the dialogue text directly here or use raw text if you are using a language file. This is an optional property, but without it your NPC dialogue box will be empty.
        text: Option<String>,
}

impl Scene {
    pub fn new(scene_tag: String) -> Self {
        Self {
            buttons: vec![],
            npc_name: None,
            on_close_commands: None,
            on_open_commands: None,
            scene_tag,
            text: None,
        }
    }


    fn add_button(&mut self, button: Button) {
        self.buttons.push(button);
    }




    fn set_npc_name(&mut self, npc_name: String) {
        self.npc_name = Some(npc_name);
    }

    fn get_npc_name(&self) -> Option<&String> {
        self.npc_name.as_ref()
    }
}

use crate::command::Command;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// This is where you can create buttons for your NPC.
struct Button {

        /// Set the text that is going to be displayed on your NPC’s button.
        name: String,

        /// allows you to add commands which will be run in-game when the button is pressed.
        commands: Vec<Command>,
}



// impl functions which is a vec of commands but if one command fails the rest will not be executed
// maybe command can be a trait which other commands can implement


impl Button {
    pub fn new(name: String) -> Self {
        Self {
            name,
            commands: vec![],
        }
    }

    fn add_command(&mut self, command: Command) {
        self.commands.push(command);
    }

    fn get_commands(&self) -> &Vec<Command> {
        &self.commands
    }

    fn get_name(&self) -> &String {
        &self.name
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn set_commands(&mut self, commands: Vec<Command>) {
        self.commands = commands;
    }


}




struct RawText {
    text: String,
}


enum Text {
    RawText(RawText),
    String(String),
}
