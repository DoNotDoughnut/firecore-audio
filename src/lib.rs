use serde::{Deserialize, Serialize};

pub mod music;
pub mod sound;

pub mod serialized;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Audio<ID> {
    pub id: ID,
    pub name: String,
}

impl<ID> Audio<ID> {

    pub fn id(&self) -> &ID {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

}