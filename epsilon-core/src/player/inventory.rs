use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inventory {
    slots: Vec<i16>,
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            slots: Vec::new(),
        }
    }

    pub fn items(&self) -> &[i16] {
        &self.slots
    }

    pub fn add(&mut self, item: i16) {
        self.slots.push(item);
    }

    pub fn remove(&mut self, item: i16) -> bool {
        if let Some(index) = self.slots.iter().position(|&id| id == item) {
            self.slots.remove(index);
            true
        } else {
            false
        }
    }

    pub fn contains(&self, item: i16) -> bool {
        self.slots.contains(&item)
    }

    pub fn clear(&mut self) {
        self.slots.clear();
    }

    pub fn len(&self) -> usize {
        self.slots.len()
    }

    pub fn is_empty(&self) -> bool {
        self.slots.is_empty()
    }
}