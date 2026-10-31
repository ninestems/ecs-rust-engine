use crate::World;
use crate::entity::Entity;

pub trait Bundle {
    fn insert_bundle(self, entity: &Entity, world: &mut World);
}

impl<A: 'static> Bundle for (A,) {
    fn insert_bundle(self, entity: &Entity, world: &mut World) {
        let (component,) = self;
        world.insert(entity, component);
    }
}

impl<A, B> Bundle for (A, B)
where
    A: 'static,
    B: 'static,
{
    fn insert_bundle(self, entity: &Entity, world: &mut World) {
        let (component_a, component_b) = self;
        world.insert(entity, component_a);
        world.insert(entity, component_b);
    }
}
