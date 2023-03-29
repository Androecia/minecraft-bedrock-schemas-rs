use crate::{general::identifier::namespace::NamespaceIdentifier, molang};
use serde::{Deserialize, Serialize};

/// Force specific decorative features (trees, plants, etc.) to appear in this Biome, regardless of normal decoration rules.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForcedFeatures {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_sky_pass: Option<Vec<Iteration>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_surface_pass: Option<Vec<Iteration>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_underground_pass: Option<Vec<Iteration>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_sky_pass: Option<Vec<Iteration>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_surface_pass: Option<Vec<Iteration>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    before_underground_pass: Option<Vec<Iteration>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    final_pass: Option<Vec<Iteration>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    first_pass: Option<Vec<Iteration>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    surface_pass: Option<Vec<Iteration>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sky_pass: Option<Vec<Iteration>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    underground_pass: Option<Vec<Iteration>>,
}

/// A single iteration of a feature placement.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Iteration {
    /// The order in which coordinates will be evaluated. Should be used when a coordinate depends on another. If omitted, defaults to `xzy`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coordinate_eval_order: Option<CoordinateEvalOrder>,

    /// UNDOCUMENTED.
    pub identifier: NamespaceIdentifier<Iteration>,

    /// Number of scattered positions to generate.
    pub iterations: molang::Number,

    /// UNDOCUMENTED.
    pub places_feature: String,

    /// Probability numerator / denominator that this scatter will occur.  Not evaluated each iteration; either no iterations will run, or all will.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scatter_chance: Option<ScatterChance>,
    /// Expression for the coordinate (evaluated each iteration).  Mutually exclusive with random distribution object below.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<Coordinate>,

    /// Expression for the coordinate (evaluated each iteration).  Mutually exclusive with random distribution object below.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<Coordinate>,

    /// Expression for the coordinate (evaluated each iteration).  Mutually exclusive with random distribution object below.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub z: Option<Coordinate>,
}
/// Probability numerator / denominator that this scatter will occur.  Not evaluated each iteration; either no iterations will run, or all will.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ScatterChance {
    Rational { numerator: i64, denominator: i64 },
    Number(molang::Number),
}

/// The order in which coordinates will be evaluated. Should be used when a coordinate depends on another. If omitted, defaults to `xzy`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CoordinateEvalOrder {
    #[serde(rename = "xyz")]
    Xyz,
    #[serde(rename = "xzy")]
    Xzy,
    #[serde(rename = "yxz")]
    Yxz,
    #[serde(rename = "yzx")]
    Yzx,
    #[serde(rename = "zxy")]
    Zxy,
    #[serde(rename = "zyx")]
    Zyx,
}

/// Expression for the coordinate (evaluated each iteration).  Mutually exclusive with random distribution object below.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Coordinate {
    String(String),
    Number(f64),
    Distribution {
        distribution: Distribution,
        extent: Extent,
        grid_offset: Option<i64>,
        step_size: Option<i64>,
    },
}
/// Type of distribution - uniform random, gaussian (centered in the range), or grid (either fixed-step or jittered).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Distribution {
    #[serde(rename = "uniform")]
    Uniform,
    #[serde(rename = "gaussian")]
    Gaussian,
    #[serde(rename = "inverse_gaussian")]
    InverseGaussian,
    #[serde(rename = "triangle")]
    Triangle,
    #[serde(rename = "fixed_grid")]
    FixedGrid,
    #[serde(rename = "jittered_grid")]
    JitteredGrid,
}

/// Lower bound (inclusive) of the scatter range, as an offset from the input point to scatter around.
/// Upper bound (inclusive) of the scatter range, as an offset from the input point to scatter around.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Extent(molang::Number, molang::Number);
