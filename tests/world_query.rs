use ecs_rust_engine as ecs;

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
fn integration_test_world_query_with_component() {
    let mut world = ecs::World::new();
    world.spawn_with((Position { x: 0.3, y: 0.4 },));
    world.spawn_with((Position { x: 0.4, y: 0.5 }, Velocity { x: 4.4, y: 4.5 }));
    let entities = world.query::<Position>();
    assert_eq!(entities.len(), 2);
}

#[test]
fn integration_test_world_query_with_several_components() {
    let mut world = ecs::World::new();
    let entity = world.spawn_with((Position { x: 0.3, y: 0.4 },));
    world.spawn_with((Velocity { x: 4.4, y: 4.5 },));
    let entities = world.query::<Position>();
    assert_eq!(entities.len(), 1);
    assert_eq!(entities[0].0, entity);
}

#[test]
fn integration_test_world_query_with_correct_component() {
    let mut world = ecs::World::new();
    let entity_1 = world.spawn_with((Position { x: 0.3, y: 0.4 },));
    let entity_2 = world.spawn_with((Position { x: 0.4, y: 0.5 }, Velocity { x: 4.4, y: 4.5 }));
    let entities = world.query::<Position>();
    assert_eq!(entities.len(), 2);

    assert!(
        entities
            .iter()
            .any(|data| data.0 == entity_1 && data.1 == &Position { x: 0.3, y: 0.4 })
    );

    assert!(
        entities
            .iter()
            .any(|data| data.0 == entity_2 && data.1 == &Position { x: 0.4, y: 0.5 })
    );
}

#[test]
fn test_world_query_two_components() {
    let mut world = ecs::World::new();
    let entity_1 = world.spawn_with((Position { x: 0.3, y: 0.4 }, Velocity { x: 4.4, y: 4.5 }));
    let components = world.query2::<Position, Velocity>();
    assert_eq!(components.len(), 1);

    assert!(components.iter().any(|data| data.0 == entity_1
        && data.1 == &Position { x: 0.3, y: 0.4 }
        && data.2 == &Velocity { x: 4.4, y: 4.5 }));
}

#[test]
fn test_world_query_only_with_several_components() {
    let mut world = ecs::World::new();
    let entity_1 = world.spawn_with((Position { x: 0.3, y: 0.4 }, Velocity { x: 4.4, y: 4.5 }));
    world.spawn_with((Position { x: 1.4, y: 1.5 },));
    world.spawn_with((Velocity { x: 2.3, y: 2.4 },));
    let components = world.query2::<Position, Velocity>();

    assert_eq!(components.len(), 1);

    assert!(components.iter().any(|data| data.0 == entity_1
        && data.1 == &Position { x: 0.3, y: 0.4 }
        && data.2 == &Velocity { x: 4.4, y: 4.5 }));
}

#[test]
fn test_world_query_several_components_empty_result() {
    let mut world = ecs::World::new();
    world.spawn_with((Position { x: 0.3, y: 0.4 },));
    world.spawn_with((Velocity { x: 2.3, y: 2.4 },));
    let components = world.query2::<Position, Velocity>();

    assert!(components.is_empty());
}

#[test]
fn test_world_query_first_mutable_components() {
    let mut world = ecs::World::new();
    let entity_1 = world.spawn_with((Position { x: 0.3, y: 0.4 }, Velocity { x: 4.4, y: 4.5 }));
    {
        let mut components = world.query2_mut::<Position, Velocity>();
        assert_eq!(components.len(), 1);
        components[0].1.x = 1.0;
        components[0].1.y = 2.0;
    }

    let position = world
        .get::<Position>(&entity_1)
        .expect("position should exist");

    assert_eq!(position, &Position { x: 1.0, y: 2.0 });
}

#[test]
fn test_world_query_none_components_with_first() {
    let mut world = ecs::World::new();
    world.spawn_with((Position { x: 0.3, y: 0.4 },));
    let components = world.query2_mut::<Position, Velocity>();

    assert_eq!(components.len(), 0);
}

#[test]
fn test_world_query_none_components_with_second() {
    let mut world = ecs::World::new();
    world.spawn_with((Velocity { x: 0.3, y: 0.4 },));
    let components = world.query2_mut::<Position, Velocity>();

    assert_eq!(components.len(), 0);
}
