use std::collections;

pub mod animation;
pub mod animation_controllers;
pub mod biome;
pub mod block;
pub mod dialogue;
pub mod entities;
pub mod item;

use std::{collections::HashMap, fs, path::PathBuf};
pub struct BehaviorPack {
    pub animation_controllers: Vec<animation_controllers::AnimationController>,
    pub animation_states: Vec<animation::Animations>,
    pub biomes: Vec<biome::Biome>,
    pub dialogue: Vec<dialogue::Dialogue>,
}

pub struct RawBehaviorPack {
    pub animation_controllers: HashMap<PathBuf, serde_json::Value>,
    pub animation_states: HashMap<PathBuf, serde_json::Value>,
    pub biomes: HashMap<PathBuf, serde_json::Value>,
    pub dialogue: HashMap<PathBuf, serde_json::Value>,
}

fn read_dir_with_ext(path: PathBuf, ext: &str) -> HashMap<PathBuf, String> {
    let mut map = HashMap::new();

    for entry in fs::read_dir(path).unwrap() {
        let path = entry.unwrap().path();
        let contents = fs::read_to_string(path.clone()).unwrap();

        if path.extension().unwrap().to_str().unwrap() != ext {
            continue;
        }
        map.insert(path, contents);
    }
    map
}

fn dir_map_to_path_value_map(map: HashMap<PathBuf, String>) -> HashMap<PathBuf, serde_json::Value> {
    let mut new_map = HashMap::new();
    for (path, contents) in map {
        let value: serde_json::Value = serde_json::from_str(&contents).unwrap();
        new_map.insert(path, value);
    }

    new_map
}

impl From<PathBuf> for RawBehaviorPack {
    fn from(path: PathBuf) -> Self {
        let mut beh_pack = RawBehaviorPack {
            animation_controllers: HashMap::new(),
            animation_states: HashMap::new(),
            biomes: HashMap::new(),
            dialogue: HashMap::new(),
        };

        // read the dir for each module and read the files and add the file path and the file contents to the hashmap for each type, the name of the struct key corresponds to the name of the dir make sure to filter out all files but json files

        let animation_controllers_path = path.join("animation_controllers");
        beh_pack.animation_controllers =
            dir_map_to_path_value_map(read_dir_with_ext(animation_controllers_path, "json"));
        let animation_states_path = path.join("animations");
        let biomes_path = path.join("biomes");
        let dialogue_path = path.join("dialogue");

        beh_pack.animation_states =
            dir_map_to_path_value_map(read_dir_with_ext(animation_states_path, "json"));

        beh_pack.biomes = dir_map_to_path_value_map(read_dir_with_ext(biomes_path, "json"));

        beh_pack.dialogue = dir_map_to_path_value_map(read_dir_with_ext(dialogue_path, "json"));

        beh_pack
    }
}
