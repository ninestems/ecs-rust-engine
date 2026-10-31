use ecs_rust_engine as ecs;

#[test]
fn integration_test_create_world() {
    let _world = ecs::World::new();
}

#[test]
fn integration_test_world_spawn_and_get_entity() {
    let mut world = ecs::World::new();
    let _entity: ecs::Entity = world.spawn();
}

#[test]
fn integration_test_world_spawn_several_entity() {
    let mut world = ecs::World::new();
    let e1 = world.spawn();
    let e2 = world.spawn();
    assert_ne!(e1, e2);
}

#[derive(Debug, PartialEq)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Debug, PartialEq)]
struct Velocity {
    x: f32,
    y: f32,
}

#[test]
fn integration_test_insert_and_read_component() {
    let mut world = ecs::World::new();
    let entity_position = world.spawn();
    world.insert(&entity_position, Position { x: 0.0, y: 0.0 });
    let component_position = world
        .get::<Position>(&entity_position)
        .expect("position should exist");
    assert_eq!(component_position, &Position { x: 0.0, y: 0.0 });
}

#[test]
fn integration_test_insert_and_read_different_components() {
    let mut world = ecs::World::new();
    let entity_position = world.spawn();
    world.insert(&entity_position, Position { x: 0.0, y: 0.0 });
    world.insert(&entity_position, Velocity { x: 0.0, y: 0.0 });
    let component_position = world
        .get::<Position>(&entity_position)
        .expect("position should exist");
    let component_velocity = world
        .get::<Velocity>(&entity_position)
        .expect("velocity should exist");
    assert_eq!(component_position, &Position { x: 0.0, y: 0.0 });
    assert_eq!(component_velocity, &Velocity { x: 0.0, y: 0.0 });
}

#[test]
fn integration_test_world_does_not_have_component() {
    let mut world = ecs::World::new();
    let entity_position = world.spawn();
    world.insert(&entity_position, Position { x: 0.0, y: 0.0 });
    let component = world.get::<Velocity>(&entity_position);
    assert!(component.is_none())
}

#[test]
fn integration_test_world_does_not_mix_components_between_entities() {
    let mut world = ecs::World::new();
    let entity_position = world.spawn();
    let entity_another = world.spawn();
    world.insert(&entity_position, Position { x: 0.0, y: 0.0 });
    let component = world.get::<Position>(&entity_another);
    assert!(component.is_none())
}

#[test]
fn integration_test_world_get_mut_component_for_edit() {
    let mut world = ecs::World::new();
    let entity_position = world.spawn();
    world.insert(&entity_position, Position { x: 0.0, y: 0.0 });
    {
        let position = world
            .get_mut::<Position>(&entity_position)
            .expect("position should exist");
        position.x = 1.0;
        position.y = 2.0;
    }
    let component = world
        .get::<Position>(&entity_position)
        .expect("position should exist");
    assert_eq!(component, &Position { x: 1.0, y: 2.0 })
}

#[test]
fn integration_test_world_get_none_component_for_edit() {
    let mut world = ecs::World::new();
    let entity_position = world.spawn();
    let component = world.get_mut::<Position>(&entity_position);
    assert!(component.is_none())
}

#[test]
fn integration_test_world_get_none_component_for_non_insert() {
    let mut world = ecs::World::new();
    let entity_position = world.spawn();
    world.insert(&entity_position, Position { x: 0.0, y: 0.0 });
    let component = world.get_mut::<Velocity>(&entity_position);
    assert!(component.is_none())
}

#[test]
fn integration_test_world_spawns_several_components() {
    let mut world = ecs::World::new();
    let entity = world.spawn_with((Position { x: 0.3, y: 0.4 },));
    let position = world
        .get::<Position>(&entity)
        .expect("position should exist");
    assert_eq!(position, &Position { x: 0.3, y: 0.4 });
}

#[test]
fn integration_test_world_spawns_two_components() {
    let mut world = ecs::World::new();
    let entity = world.spawn_with((Position { x: 1.0, y: 2.0 }, Velocity { x: 3.0, y: 4.0 }));
    let component_position = world
        .get::<Position>(&entity)
        .expect("position should exist");

    assert_eq!(component_position, &Position { x: 1.0, y: 2.0 });

    let component_velocity = world
        .get::<Velocity>(&entity)
        .expect("velocity should exist");

    assert_eq!(component_velocity, &Velocity { x: 3.0, y: 4.0 });
}
