// TODO: Make a macro to generate this
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Vanilla {
    #[serde(rename = "bamboo")]
    Bamboo,
    #[serde(rename = "basalt_deltas")]
    BasaltDeltas,
    #[serde(rename = "beach")]
    Beach,
    #[serde(rename = "bee_habitat")]
    BeeHabitat,
    #[serde(rename = "birch")]
    Birch,
    #[serde(rename = "caves")]
    Caves,
    #[serde(rename = "cold")]
    Cold,
    #[serde(rename = "crimson_forest")]
    CrimsonForest,
    #[serde(rename = "deep")]
    Deep,
    #[serde(rename = "desert")]
    Desert,
    #[serde(rename = "dripstone_caves")]
    DripstoneCaves,
    #[serde(rename = "edge")]
    Edge,
    #[serde(rename = "extreme_hills")]
    ExtremeHills,
    #[serde(rename = "flower_forest")]
    FlowerForest,
    #[serde(rename = "forest")]
    Forest,
    #[serde(rename = "frozen")]
    Frozen,
    #[serde(rename = "frozen_peaks")]
    FrozenPeaks,
    #[serde(rename = "grove")]
    Grove,
    #[serde(rename = "hills")]
    Hills,
    #[serde(rename = "ice")]
    Ice,
    #[serde(rename = "ice_plains")]
    IcePlains,
    #[serde(rename = "jagged_peaks")]
    JaggedPeaks,
    #[serde(rename = "jungle")]
    Jungle,
    #[serde(rename = "lukewarm")]
    Lukewarm,
    #[serde(rename = "lush_caves")]
    LushCaves,
    #[serde(rename = "meadow")]
    Meadow,
    #[serde(rename = "mega")]
    Mega,
    #[serde(rename = "mesa")]
    Mesa,
    #[serde(rename = "monster")]
    Monster,
    #[serde(rename = "mooshroom_island")]
    MooshroomIsland,
    #[serde(rename = "mountain")]
    Mountain,
    #[serde(rename = "mountains")]
    Mountains,
    #[serde(rename = "mutated")]
    Mutated,
    #[serde(rename = "nether")]
    Nether,
    #[serde(rename = "nether_wastes")]
    NetherWastes,
    #[serde(rename = "netherwart_forest")]
    NetherwartForest,
    #[serde(rename = "no_legacy_worldgen")]
    NoLegacyWorldgen,
    #[serde(rename = "ocean")]
    Ocean,
    #[serde(rename = "overworld")]
    Overworld,
    #[serde(rename = "overworld_generation")]
    OverworldGeneration,
    #[serde(rename = "plains")]
    Plains,
    #[serde(rename = "plateau")]
    Plateau,
    #[serde(rename = "rare")]
    Rare,
    #[serde(rename = "river")]
    River,
    #[serde(rename = "roofed")]
    Roofed,
    #[serde(rename = "savanna")]
    Savanna,
    #[serde(rename = "shore")]
    Shore,
    #[serde(rename = "snowy_slopes")]
    SnowySlopes,
    #[serde(rename = "soulsand_valley")]
    SoulsandValley,
    #[serde(rename = "spawn_endermen")]
    SpawnEndermen,
    #[serde(rename = "spawn_few_piglins")]
    SpawnFewPiglins,
    #[serde(rename = "spawn_few_zombified_piglins")]
    SpawnFewZombifiedPiglins,
    #[serde(rename = "spawn_ghast")]
    SpawnGhast,
    #[serde(rename = "spawn_magma_cubes")]
    SpawnMagmaCubes,
    #[serde(rename = "spawn_many_magma_cubes")]
    SpawnManyMagmaCubes,
    #[serde(rename = "spawn_piglin")]
    SpawnPiglin,
    #[serde(rename = "spawn_zombified_piglin")]
    SpawnZombifiedPiglin,
    #[serde(rename = "stone")]
    Stone,
    #[serde(rename = "swamp")]
    Swamp,
    #[serde(rename = "taiga")]
    Taiga,
    #[serde(rename = "the_end")]
    TheEnd,
    #[serde(rename = "warm")]
    Warm,
    #[serde(rename = "warped_forest")]
    WarpedForest,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub struct BiomeTag;
