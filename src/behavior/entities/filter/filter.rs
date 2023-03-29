

use super::filters::*;

use serde::{Deserialize, Serialize};

use std::rc::Rc;


/*
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.filters",
  "title": "Filters",
  "$ref": "#/definitions/groups_spec",
  "examples": [{ "test": "is_family", "subject": "other", "value": "example" }, { "test": "has_tag", "value": "example" }, []],
  "definitions": {
    "filters_spec": {
      "defaultSnippets": [
        {
          "label": "New Test",
          "body": { "test": "$1", "value": "$2" }
        },
        {
          "label": "New All_of Test",
          "body": {
            "all_of": [
              { "test": "$1", "value": "$2" },
              { "test": "$3", "value": "$4" }
            ]
          }
        },
        {
          "label": "New Any_of Test",
          "body": {
            "any_of": [
              { "test": "$1", "value": "$2" },
              { "test": "$3", "value": "$4" }
            ]
          }
        },
        {
          "label": "New None_of Test",
          "body": {
            "none_of": [
              { "test": "$1", "value": "$2" },
              { "test": "$3", "value": "$4" }
            ]
          }
        }
      ],
      "examples": [{ "all_of": [{}] }, { "any_of": [{}] }, { "none_of": [{}] }],
      "oneOf": [
        {
          "propertyNames": { "enum": ["all_of", "any_of", "none_of"] },
          "properties": {
            "all_of": {
              "title": "All Of",
              "description": "All tests in an `all_of` group must pass in order for the group to pass.",
              "$ref": "#/definitions/groups_spec"
            },
            "any_of": {
              "title": "Any Of",
              "description": "One or more tests in an `any_of` group must pass in order for the group to pass.",
              "$ref": "#/definitions/groups_spec"
            },
            "none_of": {
              "title": "None Of",
              "description": "All tests in a `none_of` group must fail in order for the group to pass.",
              "$ref": "#/definitions/groups_spec"
            }
          }
        },
        {
          "required": ["test"],
          "properties": {
            "all_of": {
              "title": "All Of",
              "description": "All tests in an `all_of` group must pass in order for the group to pass.",
              "$ref": "#/definitions/groups_spec"
            },
            "any_of": {
              "title": "Any Of",
              "description": "One or more tests in an `any_of` group must pass in order for the group to pass.",
              "$ref": "#/definitions/groups_spec"
            },
            "none_of": {
              "title": "None Of",
              "description": "All tests in a `none_of` group must fail in order for the group to pass.",
              "$ref": "#/definitions/groups_spec"
            }
          },
          "allOf": [
            { "if": { "properties": { "test": { "const": "bool_property" } } }, "then": { "$ref": "./filters/bool_property.json" } },
            { "if": { "properties": { "test": { "const": "clock_time" } } }, "then": { "$ref": "./filters/clock_time.json" } },
            { "if": { "properties": { "test": { "const": "distance_to_nearest_player" } } }, "then": { "$ref": "./filters/distance_to_nearest_player.json" } },
            { "if": { "properties": { "test": { "const": "enum_property" } } }, "then": { "$ref": "./filters/enum_property.json" } },
            { "if": { "properties": { "test": { "const": "float_property" } } }, "then": { "$ref": "./filters/float_property.json" } },
            { "if": { "properties": { "test": { "const": "has_ability" } } }, "then": { "$ref": "./filters/has_ability.json" } },
            { "if": { "properties": { "test": { "const": "has_biome_tag" } } }, "then": { "$ref": "./filters/has_biome_tag.json" } },
            { "if": { "properties": { "test": { "const": "has_component" } } }, "then": { "$ref": "./filters/has_component.json" } },
            { "if": { "properties": { "test": { "const": "has_container_open" } } }, "then": { "$ref": "./filters/has_container_open.json" } },
            { "if": { "properties": { "test": { "const": "has_damage" } } }, "then": { "$ref": "./filters/has_damage.json" } },
            { "if": { "properties": { "test": { "const": "has_equipment" } } }, "then": { "$ref": "./filters/has_equipment.json" } },
            { "if": { "properties": { "test": { "const": "has_mob_effect" } } }, "then": { "$ref": "./filters/has_mob_effect.json" } },
            { "if": { "properties": { "test": { "const": "has_nametag" } } }, "then": { "$ref": "./filters/has_nametag.json" } },
            { "if": { "properties": { "test": { "const": "has_property" } } }, "then": { "$ref": "./filters/has_property.json" } },
            { "if": { "properties": { "test": { "const": "has_ranged_weapon" } } }, "then": { "$ref": "./filters/has_ranged_weapon.json" } },
            { "if": { "properties": { "test": { "const": "has_silk_touch" } } }, "then": { "$ref": "./filters/has_silk_touch.json" } },
            { "if": { "properties": { "test": { "const": "has_tag" } } }, "then": { "$ref": "./filters/has_tag.json" } },
            { "if": { "properties": { "test": { "const": "has_target" } } }, "then": { "$ref": "./filters/has_target.json" } },
            { "if": { "properties": { "test": { "const": "has_trade_supply" } } }, "then": { "$ref": "./filters/has_trade_supply.json" } },
            { "if": { "properties": { "test": { "const": "hourly_clock_time" } } }, "then": { "$ref": "./filters/hourly_clock_time.json" } },
            { "if": { "properties": { "test": { "const": "in_block" } } }, "then": { "$ref": "./filters/in_block.json" } },
            { "if": { "properties": { "test": { "const": "in_caravan" } } }, "then": { "$ref": "./filters/in_caravan.json" } },
            { "if": { "properties": { "test": { "const": "in_clouds" } } }, "then": { "$ref": "./filters/in_clouds.json" } },
            { "if": { "properties": { "test": { "const": "in_contact_with_water" } } }, "then": { "$ref": "./filters/in_contact_with_water.json" } },
            { "if": { "properties": { "test": { "const": "in_lava" } } }, "then": { "$ref": "./filters/in_lava.json" } },
            { "if": { "properties": { "test": { "const": "in_nether" } } }, "then": { "$ref": "./filters/in_nether.json" } },
            { "if": { "properties": { "test": { "const": "in_water_or_rain" } } }, "then": { "$ref": "./filters/in_water_or_rain.json" } },
            { "if": { "properties": { "test": { "const": "in_water" } } }, "then": { "$ref": "./filters/in_water.json" } },
            { "if": { "properties": { "test": { "const": "inactivity_timer" } } }, "then": { "$ref": "./filters/inactivity_timer.json" } },
            { "if": { "properties": { "test": { "const": "int_property" } } }, "then": { "$ref": "./filters/int_property.json" } },
            { "if": { "properties": { "test": { "const": "is_altitude" } } }, "then": { "$ref": "./filters/is_altitude.json" } },
            { "if": { "properties": { "test": { "const": "is_avoiding_mobs" } } }, "then": { "$ref": "./filters/is_avoiding_mobs.json" } },
            { "if": { "properties": { "test": { "const": "is_biome" } } }, "then": { "$ref": "./filters/is_biome.json" } },
            { "if": { "properties": { "test": { "const": "is_block" } } }, "then": { "$ref": "./filters/is_block.json" } },
            { "if": { "properties": { "test": { "const": "is_brightness" } } }, "then": { "$ref": "./filters/is_brightness.json" } },
            { "if": { "properties": { "test": { "const": "is_climbing" } } }, "then": { "$ref": "./filters/is_climbing.json" } },
            { "if": { "properties": { "test": { "const": "is_color" } } }, "then": { "$ref": "./filters/is_color.json" } },
            { "if": { "properties": { "test": { "const": "is_daytime" } } }, "then": { "$ref": "./filters/is_daytime.json" } },
            { "if": { "properties": { "test": { "const": "is_difficulty" } } }, "then": { "$ref": "./filters/is_difficulty.json" } },
            { "if": { "properties": { "test": { "const": "is_family" } } }, "then": { "$ref": "./filters/is_family.json" } },
            { "if": { "properties": { "test": { "const": "is_game_rule" } } }, "then": { "$ref": "./filters/is_game_rule.json" } },
            { "if": { "properties": { "test": { "const": "is_humid" } } }, "then": { "$ref": "./filters/is_humid.json" } },
            { "if": { "properties": { "test": { "const": "is_immobile" } } }, "then": { "$ref": "./filters/is_immobile.json" } },
            { "if": { "properties": { "test": { "const": "is_in_village" } } }, "then": { "$ref": "./filters/is_in_village.json" } },
            { "if": { "properties": { "test": { "const": "is_leashed_to" } } }, "then": { "$ref": "./filters/is_leashed_to.json" } },
            { "if": { "properties": { "test": { "const": "is_leashed" } } }, "then": { "$ref": "./filters/is_leashed.json" } },
            { "if": { "properties": { "test": { "const": "is_mark_variant" } } }, "then": { "$ref": "./filters/is_mark_variant.json" } },
            { "if": { "properties": { "test": { "const": "is_missing_health" } } }, "then": { "$ref": "./filters/is_missing_health.json" } },
            { "if": { "properties": { "test": { "const": "is_moving" } } }, "then": { "$ref": "./filters/is_moving.json" } },
            { "if": { "properties": { "test": { "const": "is_owner" } } }, "then": { "$ref": "./filters/is_owner.json" } },
            { "if": { "properties": { "test": { "const": "is_persistent" } } }, "then": { "$ref": "./filters/is_persistent.json" } },
            { "if": { "properties": { "test": { "const": "is_riding" } } }, "then": { "$ref": "./filters/is_riding.json" } },
            { "if": { "properties": { "test": { "const": "is_skin_id" } } }, "then": { "$ref": "./filters/is_skin_id.json" } },
            { "if": { "properties": { "test": { "const": "is_sleeping" } } }, "then": { "$ref": "./filters/is_sleeping.json" } },
            { "if": { "properties": { "test": { "const": "is_sneaking" } } }, "then": { "$ref": "./filters/is_sneaking.json" } },
            { "if": { "properties": { "test": { "const": "is_snow_covered" } } }, "then": { "$ref": "./filters/is_snow_covered.json" } },
            { "if": { "properties": { "test": { "const": "is_target" } } }, "then": { "$ref": "./filters/is_target.json" } },
            { "if": { "properties": { "test": { "const": "is_temperature_type" } } }, "then": { "$ref": "./filters/is_temperature_type.json" } },
            { "if": { "properties": { "test": { "const": "is_temperature_value" } } }, "then": { "$ref": "./filters/is_temperature_value.json" } },
            { "if": { "properties": { "test": { "const": "is_underground" } } }, "then": { "$ref": "./filters/is_underground.json" } },
            { "if": { "properties": { "test": { "const": "is_underwater" } } }, "then": { "$ref": "./filters/is_underwater.json" } },
            { "if": { "properties": { "test": { "const": "is_variant" } } }, "then": { "$ref": "./filters/is_variant.json" } },
            { "if": { "properties": { "test": { "const": "is_visible" } } }, "then": { "$ref": "./filters/is_visible.json" } },
            { "if": { "properties": { "test": { "const": "is_waterlogged" } } }, "then": { "$ref": "./filters/is_waterlogged.json" } },
            { "if": { "properties": { "test": { "const": "light_level" } } }, "then": { "$ref": "./filters/light_level.json" } },
            { "if": { "properties": { "test": { "const": "moon_intensity" } } }, "then": { "$ref": "./filters/moon_intensity.json" } },
            { "if": { "properties": { "test": { "const": "moon_phase" } } }, "then": { "$ref": "./filters/moon_phase.json" } },
            { "if": { "properties": { "test": { "const": "on_ground" } } }, "then": { "$ref": "./filters/on_ground.json" } },
            { "if": { "properties": { "test": { "const": "on_ladder" } } }, "then": { "$ref": "./filters/on_ladder.json" } },
            { "if": { "properties": { "test": { "const": "random_chance" } } }, "then": { "$ref": "./filters/random_chance.json" } },
            { "if": { "properties": { "test": { "const": "rider_count" } } }, "then": { "$ref": "./filters/rider_count.json" } },
            { "if": { "properties": { "test": { "const": "surface_mob" } } }, "then": { "$ref": "./filters/surface_mob.json" } },
            { "if": { "properties": { "test": { "const": "trusts" } } }, "then": { "$ref": "./filters/trusts.json" } },
            { "if": { "properties": { "test": { "const": "weather_at_position" } } }, "then": { "$ref": "./filters/weather_at_position.json" } },
            { "if": { "properties": { "test": { "const": "weather" } } }, "then": { "$ref": "./filters/weather.json" } },
            { "not": { "properties": { "test": { "const": "is_weather" } }, "$comment": "DEPRECATED" } }
          ]
        }
      ]
    },
    "groups_spec": {
      "oneOf": [
        { "type": "array", "items": { "$ref": "#/definitions/groups_spec" } },
        { "type": "object", "$ref": "#/definitions/filters_spec" }
      ]
    }
  }
}


 */

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Filters {
  Group(Vec<Box<Filters>>),
  Filter(Box<Filter>),
}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Filter {
  Filter(FilterTest),
  FilterGroup(FilterGroup),
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FilterGroup {
  /// All tests in an `all_of` group must pass in order for the group to pass.
  all_of:Option<Filters>,
  /// At least one test in an `any_of` group must pass in order for the group to pass.
  any_of:Option<Filters>,
  /// None of the tests in a `none_of` group must pass in order for the group to pass.
  none_of:Option<Filters>,
}

// the key test is the name of the test in snake case { "test": "is_family", "subject": "other", "value": "example" }
#[derive(Debug, Clone, PartialEq,Serialize )]
enum FilterTest {

    BoolProperty(BoolProperty),
    ClockTime(ClockTime),
    DistanceToNearestPlayer(DistanceToNearestPlayer),
    EnumProperty(EnumProperty),
    FloatProperty(FloatProperty),
    HasAbility(HasAbility),
    HasBiomeTag(HasBiomeTag),
    HasComponent(HasComponent),
    HasContainerOpen(HasContainerOpen),
    HasDamage(HasDamage),
    HasEquipment(HasEquipment),
    HasMobEffect(HasMobEffect),
    HasNametag(HasNametag),
    HasProperty(HasProperty),
    HasRangedWeapon(HasRangedWeapon),
    HasSilkTouch(HasSilkTouch),
    HasTag(HasTag),
    HasTarget(HasTarget),
    HasTradeSupply(HasTradeSupply),
    HourlyClockTime(HourlyClockTime),
    InactivityTimer(InactivityTimer),
    IntProperty(IntProperty),
    InBlock(InBlock),
    InCaravan(InCaravan),
    InClouds(InClouds),
    InContactWithWater(InContactWithWater),
    InLava(InLava),
    InNether(InNether),
    InWater(InWater),
    InWaterOrRain(InWaterOrRain),
    IsAltitude(IsAltitude),
    IsAvoidingMobs(IsAvoidingMobs),
    IsBiome(IsBiome),
    IsBlock(IsBlock),
    IsBrightness(IsBrightness),
    IsClimbing(IsClimbing),
    IsColor(IsColor),
    IsDaytime(IsDaytime),
    IsDifficulty(IsDifficulty),
    IsFamily(IsFamily),
    IsGameRule(IsGameRule),
    IsHumid(IsHumid),
    IsImmobile(IsImmobile),
    IsInVillage(IsInVillage),
    IsLeashed(IsLeashed),
    IsLeashedTo(IsLeashedTo),
    IsMarkVariant(IsMarkVariant),
    IsMissingHealth(IsMissingHealth),
    IsMoving(IsMoving),
    IsOwner(IsOwner),
    IsPersistent(IsPersistent),
    IsRiding(IsRiding),
    IsSkinId(IsSkinId),
    IsSleeping(IsSleeping),
    IsSneaking(IsSneaking),
    IsSnowCovered(IsSnowCovered),
    IsTarget(IsTarget),
    IsTemperatureType(IsTemperatureType),
    IsTemperatureValue(IsTemperatureValue),
    IsUnderground(IsUnderground),
    IsUnderwater(IsUnderwater),
    IsVariant(IsVariant),
    IsVisible(IsVisible),
    IsWaterlogged(IsWaterlogged),
    LightLevel(LightLevel),
    MoonIntensity(MoonIntensity),
    MoonPhase(MoonPhase),
    OnGround(OnGround),
    OnLadder(OnLadder),
    RandomChance(RandomChance),
    RiderCount(RiderCount),
    SurfaceMob(SurfaceMob),
    Trusts(Trusts),
    Weather(Weather),
    WeatherAtPosition(WeatherAtPosition),
}







