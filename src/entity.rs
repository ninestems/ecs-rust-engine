#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Entity {
    name: String,
}

impl Entity {
    pub fn new(name: String) -> Entity {
        Entity { name }
    }
}
