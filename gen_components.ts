import * as fs from "fs";


/*js string array from Properties List
ambient_sound_interval
can_climb
can_fly
can_power_jump
collision_box
color
color2
default_look_angle
equipment
fire_immune
floats_in_liquid
flying_speed
friction_modifier
ground_offset
input_ground_controlled
is_baby
is_charged
is_chested
is_dyeable
is_hidden_when_invisible
is_ignited
is_illager_captain
is_saddled
is_shaking
is_sheared
is_stackable
is_stunned
is_tamed
item_controllable
loot
mark_variant
push_through
scale
skin_id
sound_volume
type_family
variant
walk_animation_speed
wants_jockey */


const propList =[
	"ambient_sound_interval",
"can_climb",
	"can_fly",
	"can_power_jump",
	"collision_box",
	"color",
	"color2",
	"default_look_angle",
	"equipment",
	"fire_immune",
	"floats_in_liquid",
	"flying_speed",
	"friction_modifier",
	"ground_offset",
	"input_ground_controlled",
	"is_baby",
	"is_charged",
	"is_chested",
	"is_dyeable",
	"is_hidden_when_invisible",
	"is_ignited",
	"is_illager_captain",
	"is_saddled",
	"is_shaking",
	"is_sheared",
	"is_stackable",
	"is_stunned",
	"is_tamed",
	"item_controllable",
	"loot",
	"mark_variant",
	"push_through",
	"scale",
	"skin_id",
	"sound_volume",
	"type_family",
	"variant",
	"walk_animation_speed",
	"wants_jockey"
];

const propertiesDirectory = "src/behavior/entities/properties";
// if file name exists in array put the output file in the properties directory

const sourcePath =
	"Minecraft-bedrock-json-schemas/source/behavior/entities/format/components";
const componentsDir = "src/behavior/entities/components";

fs.readdir(sourcePath, (err, files) => {
	if (err) {
		console.error(`Error reading directory ${sourcePath}: ${err}`);
		return;
	}

	files.forEach(file => {
		if (file.endsWith(".json")) {
			const fileName = file.replace(".json", "");
			const structName = toCamelCase(fileName);

			const jsonContent = fs.readFileSync(
				`${sourcePath}/${file}`,
				"utf-8"
			);
			const rsComment = `/* Raw contents of ${file} That I want to use to make a rust Struct from this Json Schema:\n${jsonContent}*/ `;

			const rsContent = `${rsComment}use serde::{Deserialize, Serialize};\n\n#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]\npub struct ${structName} {}\n`;


				let destFilePath = `${componentsDir}/${fileName}.rs`;

				if  (propList.includes(fileName)) {
					destFilePath = `${propertiesDirectory}/${fileName}.rs`;
				}


			fs.access(destFilePath, fs.constants.F_OK, err => {
				if (err) {
					fs.writeFile(destFilePath, rsContent, err => {
						if (err) {
							console.error(
								`Error writing file ${destFilePath}: ${err}`
							);
						} else {
							console.log(`Created file ${destFilePath}`);
						}
					});
				}
			});
		}
	});
});

function toCamelCase(str: string): string {


	return str.replace(/^[a-z]/, (c) => c.toUpperCase()).replace(/[-_](.)/g, (_, c) => c.toUpperCase());
}
