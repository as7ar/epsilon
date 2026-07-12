use std::collections::HashMap;
use crate::item::Item;

pub struct ItemRegistry {
    items: HashMap<i16, Item>,
}

impl ItemRegistry {
    pub fn new() -> Self {
        Self {
            items: HashMap::new()
        }
    }

    pub fn get(&self, id: i16) -> Option<&Item> {
        self.items.get(&id)
    }

    pub fn register(&mut self, item: Item) -> Option<Item> {
        self.items.insert(
            item.id,
            item
        )
    }
}