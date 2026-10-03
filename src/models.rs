use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Item {
    pub id: u32,
    pub name: String, 
    pub location: String, 
    pub category: String, 
    pub notes: String,
    pub photo_path: Option<String>,
}

impl Item {
    pub fn new(id: u32, name: String, location: String, category: String, notes: String, photo_path: Option<String>) -> Self {
        Self {
            id,
            name,
            location,
            category, 
            notes,
            photo_path,
        }
    }
}
