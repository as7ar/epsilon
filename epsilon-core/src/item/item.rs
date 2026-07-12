use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Item {
    pub id: i16,
    pub name: String,
    pub description: String,
    pub stack_size: u16,
    pub texture: String,
}

impl Item {
    fn new(id: i16, name: String, description: String, stack_size: u16, texture: String) -> Self {
        Self {
            id,name, description,
            stack_size,
            texture
        }
    }
}