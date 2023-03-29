use crate::general::{rawtext::RawText, version::Version};
use crate::slash_command::SlashCommand;

use serde::{Deserialize, Serialize};

/// Specifies the dialogue of an npc.
#[derive(Clone, Serialize, Deserialize)]
pub struct Dialogue {
    pub format_version: Version,

    /// The different scenes.
    #[serde(rename = "minecraft:npc_dialogue")]
    pub npc_dialogue: NpcDialogue,
}

#[derive(Clone, Serialize, Deserialize)]
/// Specifies the dialogue of an npc.
pub struct NpcDialogue {
    /// The different scenes.
    pub scenes: Vec<Scene>,
}

#[derive(Serialize, Deserialize, Clone)]
/// A single scene specification.
pub struct Scene {
    /// This is the name you will use to call this scene in-game. This is a required property.
    pub scene_tag: String,

    /// This is where you can create buttons for your NPC.
    pub buttons: Option<Vec<Button>>,

    /// This is where you can add or change a name for your NPC dialogue box. This is an optional property that is useful for dynamically changing NPC names.
    pub npc_name: Option<String>,

    /// This is where you can define which commands will fire when the NPC dialogue box closes.
    pub on_close_commands: Option<Vec<SlashCommand>>,

    /// This is where you can define which commands will fire when the NPC dialogue box opens.
    pub on_open_commands: Option<Vec<SlashCommand>>,

    /// This is where you enter the dialogue you want your NPC to display in-game for this scene. You can type the dialogue text directly here or use raw text if you are using a language file. This is an optional property, but without it your NPC dialogue box will be empty.
    pub text: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// This is where you can create buttons for your NPC.
pub struct Button {
    /// Set the text that is going to be displayed on your NPC’s button.
    pub name: String,

    /// allows you to add commands which will be run in-game when the button is pressed.
    pub commands: Vec<SlashCommand>,
}

// impl functions which is a vec of commands but if one command fails the rest will not be executed
// maybe command can be a trait which other commands can implement

pub enum Text {
    RawText(RawText),
    String(String),
}
