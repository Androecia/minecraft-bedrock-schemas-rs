use serde::{Deserialize, Serialize};

/// Minimum and maximum values.
/// Values are between 0.0 and 1.0.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FloatRange(f64, f64);
