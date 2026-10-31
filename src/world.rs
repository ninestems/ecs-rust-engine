use crate::bundle::Bundle;
use crate::entity::Entity;
use crate::storage::Storage;

use std::any::Any;
use std::any::TypeId;
use std::collections::HashMap;

pub struct World {
    counter: usize,
    entities: Vec<Entity>,
    components: HashMap<TypeId, Box<dyn Any>>,
}

impl World {
    pub fn new() -> Self {
        World {
            counter: 0,
            entities: Vec::new(),
            components: HashMap::new(),
        }
    }

    pub fn spawn(&mut self) -> Entity {
        self.counter += 1;
        self.entities.push(Entity::new(self.counter.to_string()));
        self.entities[self.counter - 1].clone()
    }

    pub fn spawn_with<B: Bundle>(&mut self, bundles: B) -> Entity {
        let entity = self.spawn();
        bundles.insert_bundle(&entity, self);
        entity
    }

    pub fn insert<T: 'static>(&mut self, entity: &Entity, component: T) {
        let type_id = TypeId::of::<T>();

        if !self.components.contains_key(&type_id) {
            self.components
                .insert(type_id, Box::new(Storage::<T>::new()));
        }

        let storage_any = self
            .components
            .get_mut(&type_id)
            .expect("storage should exist after insertion");

        let storage = storage_any
            .downcast_mut::<Storage<T>>()
            .expect("storage type should match TypeId");

        storage.insert(entity, component);
    }

    pub fn get<T: 'static>(&self, entity: &Entity) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        let storage_any = self.components.get(&type_id)?;

        let storage = storage_any
            .downcast_ref::<Storage<T>>()
            .expect("storage type should match TypeId");
        storage.get(entity)
    }

    pub fn get_mut<T: 'static>(&mut self, entity: &Entity) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();
        let storage_any = self.components.get_mut(&type_id)?;
        let storage = storage_any
            .downcast_mut::<Storage<T>>()
            .expect("storage type should match TypeId");
        storage.get_mut(entity)
    }

    pub fn query<T: 'static>(&self) -> Vec<(Entity, &T)> {
        let type_id = TypeId::of::<T>();
        let storage_any = self.components.get(&type_id);

        match storage_any {
            Some(storage) => {
                let storage = storage
                    .downcast_ref::<Storage<T>>()
                    .expect("storage type should match TypeId");

                let mut result: Vec<(Entity, &T)> = Vec::new();

                for element in storage.iter() {
                    result.push((element.0.clone(), element.1))
                }

                result
            }
            None => Vec::new(),
        }
    }

    pub fn query2<A: 'static, B: 'static>(&self) -> Vec<(Entity, &A, &B)> {
        let vec_a = self.query::<A>();
        let mut result = Vec::new();

        for a in vec_a.iter() {
            if let Some(component) = self.get::<B>(&a.0) {
                result.push((a.0.clone(), a.1, component));
            }
        }

        result
    }

    pub fn query2_mut<A: 'static, B: 'static>(&mut self) -> Vec<(Entity, &mut A, &B)> {
        let type_id_a = TypeId::of::<A>();
        let type_id_b = TypeId::of::<B>();

        let [storage_any_a, storage_any_b] =
            self.components.get_disjoint_mut([&type_id_a, &type_id_b]);

        let (Some(storage_any_a), Some(storage_any_b)) = (storage_any_a, storage_any_b) else {
            return Vec::new();
        };

        let storage_a: &mut Storage<A> = storage_any_a
            .downcast_mut::<Storage<A>>()
            .expect("storage type should match TypeId");

        let storage_b: &Storage<B> = storage_any_b
            .downcast_ref::<Storage<B>>()
            .expect("storage type should match TypeId");

        let mut result: Vec<(Entity, &mut A, &B)> = Vec::new();

        for element in storage_a.iter_mut() {
            if let Some(component) = storage_b.get(&element.0) {
                result.push((element.0.clone(), element.1, component));
            }
        }

        result
    }
}
