use std::collections::HashMap;

use crate::entity::Entity;

pub struct Storage<T> {
    objects: HashMap<Entity, T>,
}

impl<T> Storage<T> {
    pub fn new() -> Storage<T> {
        Storage {
            objects: HashMap::new(),
        }
    }

    pub fn get(&self, entity: &Entity) -> Option<&T> {
        self.objects.get(entity)
    }

    pub fn get_mut(&mut self, entity: &Entity) -> Option<&mut T> {
        self.objects.get_mut(entity)
    }

    pub fn insert(&mut self, entity: &Entity, object: T) {
        self.objects.insert(entity.clone(), object);
    }

    pub fn remove(&mut self, entity: &Entity) -> Option<T> {
        self.objects.remove(entity)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Entity, &T)> {
        self.objects.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&Entity, &mut T)> {
        self.objects.iter_mut()
    }
}
