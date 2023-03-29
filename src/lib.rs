pub mod behavior;
pub mod general;
pub mod molang;
pub mod pack;
pub mod resource;
pub mod shared;
pub mod slash_command;
pub mod types;
pub mod utils;

#[cfg(test)]
mod test {

    // the directory of the minecraft files
    // data/behavior_packs
    // data/resource_packs
    // data/definitions
    // data/skinpacks

    const GAME_DIR: &str = r"C:\Users\miner\Programs\MCLauncher\Minecraft-Preview-1.19.80.20";

    use super::*;
    use std::fs;
    use std::path::{Path, PathBuf};

    use json_comments::StripComments;
    use serde::{ser::SerializeMap, Deserialize, Serialize};
    use walkdir::WalkDir;

    use crate::general::identifier::namespace::NamespaceIdentifier;
    //TODO: make something for tests to verify it wont serialize incorrect files

    fn for_each_pack_in_dir(path: &str) {

        // for each pack test if it can be serialized and deserialized
    }

    pub fn test_serde_json_from_files_in_path<T>(
        paths: Vec<PathBuf>,
        ignored_ser_files: Option<Vec<String>>,
    ) where
        T: serde::de::DeserializeOwned + ?Sized + serde::Serialize,
    {
        let mut file_paths = Vec::new();
        for path in paths {
            for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
                if entry.file_type().is_file() {
                    file_paths.push(entry.path().to_str().unwrap().to_string());
                }
            }
        }

        let ignored_ser_files = ignored_ser_files.unwrap_or_default();

        for file_name in file_paths {
            let file = std::fs::File::open(file_name.clone()).unwrap();

            let reader = std::io::BufReader::new(file);

            let stripped = StripComments::new(reader);

            // println!("Parsing file: {}", file_name);

            use std::env;

            let os = env::consts::OS;

            let split = match os {
                "windows" => "\\",
                _ => "/",
            };

            let file_name_cut = &file_name.split(split).last().unwrap().to_string();

            println!("Testing File: {}", file_name_cut);

            let deser_file: T = match serde_json::from_reader(stripped) {
                Ok(file) => file,
                Err(e) => panic!("error parsing json: {}, \n File: {}", e, file_name),
            };

            let file = std::fs::File::open(file_name.clone()).unwrap();
            let reader = std::io::BufReader::new(file);

            // let stripped = StripComments::new(data.as_bytes());

            let stripped = StripComments::new(reader);

            let original_file: serde_json::Value = match serde_json::from_reader(stripped) {
                Ok(file) => file,
                Err(e) => panic!("error parsing json: {}, \n File: {}", e, file_name),
            };

            // deser_file as serde_json::Value

            let ser_file = serde_json::to_string_pretty(&deser_file).unwrap();

            // println!("Serializing file: {}", ser_file);

            let deser_file: serde_json::Value = match serde_json::from_str(&ser_file) {
                Ok(file) => file,
                Err(e) => panic!("error parsing json: {}, \n File: {}", e, file_name),
            };

            // Ignore files that are not supposed to be serialized

            // get the path split based on os

            if ignored_ser_files.contains(file_name_cut) {
                continue;
            }

            if original_file != deser_file {
                // show original file value and deser file value in the error with the file name and an error message

                text_diff::print_diff(
                    &format!("{:#?}", original_file),
                    &format!("{:#?}", deser_file),
                    "\n",
                );

                // File: {}\n------------------\nOriginal File: \n{:#?}\n------------------\n Serialized File:\n {:#?}\n------------------\n
                panic!("\n------------------\nThe serialized value != the original value \n------------------\nFile: {}\n------------------\n", file_name/*, original_file, deser_file*/);
            }

            // files.push(file);
        }
    }

    #[test]
    fn test_game_files() {
        let path = Path::new(GAME_DIR).to_path_buf().join("data");

        // for all packs in data/behavior_packs each dir is a pack NOT RECURSIVE

        for pack in fs::read_dir(path.join("behavior_packs")).unwrap() {
            let pack = pack.unwrap();
            let pack_path = pack.path();
            let pack_name = pack_path.file_name().unwrap().to_str().unwrap();
            println!("Testing behavior pack: {}", pack_name);
            test_behavior_pack(pack_path);
        }

        // for all packs in data/resource_packs each dir is a pack NOT RECURSIVE

        for pack in fs::read_dir(path.join("resource_packs")).unwrap() {
            let pack = pack.unwrap();
            let pack_path = pack.path();
            let pack_name = pack_path.file_name().unwrap().to_str().unwrap();
            println!("Testing resource pack: {}", pack_name);
            test_resource_pack(pack_path);
        }

        // in data/definitions/biomes contain the biomes, test that
        println!("Testing definitions: biomes ");
        test_serde_json_from_files_in_path::<behavior::biome::Biome>(
            vec![path.clone().join("definitions").join("biomes")],
            // these error out due to serde making 1.0 into 1
            Some(vec![
                "jagged_peaks.biome.json".to_string(),
                "stony_peaks.biome.json".to_string(),
            ]),
        );
    }

    //--------------------------------------------------------------------------------

    // Behavior Pack Tests

    //--------------------------------------------------------------------------------

    fn test_behavior_pack(path: PathBuf) {
        // for each part of a behavior pack test if it can be serialized and deserialized

        test_serde_json_from_files_in_path::<behavior::animation_controllers::AnimationControllers>(
            vec![path.clone().join("animations")],
            None,
        );

        // C:\Users\miner\Programs\MCLauncher\Minecraft-Preview-1.19.80.20\data\resource_packs\vanilla_1.15\animations\dressing_room_react_bottom_3.anim.json

        test_serde_json_from_files_in_path::<behavior::animation::Animations>(
            vec![path.clone().join("animations")],
            None,
        );

        // biomes

        test_serde_json_from_files_in_path::<behavior::biome::Biome>(
            vec![path.clone().join("biomes")],
            None,
        );

        // dialogue

        test_serde_json_from_files_in_path::<behavior::dialogue::Dialogue>(
            vec![path.clone().join("dialogues")],
            None,
        );
    }

    #[test]
    fn test_behavior_animation_controllers() {
        let paths = vec![
            Path::new(
                "./Minecraft-bedrock-json-schemas/test/files/correct/data_bp/animation_controllers",
            )
            .to_path_buf(),
            Path::new("./samples/behavior/animation_controllers").to_path_buf(),
        ];
        test_serde_json_from_files_in_path::<behavior::animation_controllers::AnimationControllers>(
            paths, None,
        );
    }
    // TODO: behavior animation tests

    // TODO: biome tests

    // TODO: dialog tests

    // --------------------------------------------------------------------------------

    // Resource Pack Tests

    // --------------------------------------------------------------------------------

    fn test_resource_pack(path: PathBuf) {
        // animation_controllers

        test_serde_json_from_files_in_path::<resource::animation_controllers::AnimationControllers>(
            vec![path.clone().join("animation_controllers")],
            None,
        );

        // animations

        test_serde_json_from_files_in_path::<resource::animations::ActorAnimations>(
            vec![path.clone().join("animations")],
            Some(vec!["dressing_room_react_bottom_3.anim.json".to_string()]),
        );

        // fogs

        test_serde_json_from_files_in_path::<resource::fog::Fog>(
            vec![path.clone().join("fogs")],
            None,
        );
    }

    #[test]
    fn serde_resource_animation_controllers() {
        let paths = vec![
            Path::new(
                "./Minecraft-bedrock-json-schemas/test/files/correct/data_rp/animation_controllers",
            )
            .to_path_buf(),
            Path::new("./bedrock-samples/resource_pack/animation_controllers").to_path_buf(),
        ];

        test_serde_json_from_files_in_path::<resource::animation_controllers::AnimationControllers>(
            paths, None,
        );
    }

    #[test]
    fn serde_resource_actor_animations() {
        let paths = vec![
            Path::new("./Minecraft-bedrock-json-schemas/test/files/correct/data_rp/animations")
                .to_path_buf(),
            Path::new("./bedrock-samples/resource_pack/animations").to_path_buf(),
        ];

        let ignored_ser_files = vec!["dressing_room_react_bottom_3.anim.json".to_string()];

        test_serde_json_from_files_in_path::<resource::animations::ActorAnimations>(
            paths,
            Some(ignored_ser_files),
        );
    }

    #[test]
    fn serde_resource_fog() {
        let paths = vec![
            Path::new("./bedrock-samples/resource_pack/fogs/").to_path_buf(),
            Path::new("./Minecraft-bedrock-json-schemas/test/files/correct/data_rp/fogs/")
                .to_path_buf(),
        ];

        test_serde_json_from_files_in_path::<resource::fog::Fog>(paths, None);
    }
}
