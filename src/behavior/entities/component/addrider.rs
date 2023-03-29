use serde::{Deserialize, Serialize};

use crate::behavior::entities::entity::event::EventIdentifier;

/// Adds a rider to the entity. Requires `minecraft:rideable.`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddRider {
    /// The entity type that will be riding this entity.
    pub entity_type: EntityIdentifier,
    /// The spawn event that will be used when the riding entity is created.
    pub spawn_event: Option<EventIdentifier>,
}
