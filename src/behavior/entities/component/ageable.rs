use serde::{Deserialize, Serialize};

/// Adds a timer for the entity to grow up. It can be accelerated by giving the entity the items it likes as defined by feedItems.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ageable {
    /// List of items that the entity drops when it grows up.
    pub drop_items: Option<Vec<ItemWeight>>,
    /// Amount of time before the entity grows up, -1 for always a baby.
    pub duration: Option<i32>,
    /// List of items that can be fed to the entity. Includes `item` for the item name and `growth` to define how much time it grows up by
    pub feed_items: Option<Vec<ItemWeight>>,
    /// Event to run when this entity grows up.
    pub grow_up: Option<EventIdentifier>,
    /// The feed item used will transform to this item upon successful interaction. Format: itemName:auxValue
    pub transform_to_item: Option<ItemIdentifierWithData>,
}

impl Ageable {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set_always_baby(&mut self) {
        self.duration = Some(-1);
    }
}

// TODO: Better way to name this?
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ItemWeight {
    Single(ItemType),
    Multiple(Vec<ItemType>),
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemWeight {
    pub item: ItemIdentifier,
    pub weight: i32,
}







#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ItemType {
    ItemIdentifier(ItemIdentifier),
    ItemWeight(ItemWeight),

}

impl Default for Ageable {
    fn default() -> Self {
        Self {
            drop_items: None,
            duration: Some(1200),
            feed_items: None,
            grow_up: None,
            transform_to_item: None,
        }
    }
}
