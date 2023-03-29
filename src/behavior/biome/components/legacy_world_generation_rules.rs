use serde::{Deserialize, Serialize};
/// Additional world generation control applicable only to legacy limited worlds.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LegacyWorldGenerationRules {}
