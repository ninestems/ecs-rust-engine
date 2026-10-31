use ecs_rust_engine as ecs;
use ecs_rust_engine::Entity;

#[derive(Debug, PartialEq)]
struct Position {
    x: f32,
    y: f32,
}

#[test]
fn integration_create_and_read_component() {
    let entity = Entity::new(String::from("hello"));
    let mut storage = ecs::Storage::<Position>::new();
    storage.insert(&entity, Position { x: 0.0, y: 0.0 });
    let position = storage.get(&entity).expect("position should exist");
    assert_eq!(position, &Position { x: 0.0, y: 0.0 });
}

#[test]
fn integration_unexisting_component() {
    let entity = Entity::new(String::from("hello"));
    let storage = ecs::Storage::<Position>::new();
    let position = storage.get(&entity);
    assert!(position.is_none());
}

#[test]
fn integration_update_component() {
    let entity = Entity::new(String::from("hello"));
    let mut storage = ecs::Storage::<Position>::new();
    storage.insert(&entity, Position { x: 0.0, y: 0.0 });

    {
        let position = storage.get_mut(&entity).expect("position should exist");
        position.x = 1.0;
        position.y = 2.0;
    }

    let position = storage.get(&entity).expect("position should exist");
    assert_eq!(position, &Position { x: 1.0, y: 2.0 });
}

#[test]
fn integration_delete_component() {
    let entity = Entity::new(String::from("hello"));
    let mut storage = ecs::Storage::<Position>::new();
    storage.insert(&entity, Position { x: 1.0, y: 2.0 });
    let position = storage.remove(&entity);
    assert_eq!(position, Some(Position { x: 1.0, y: 2.0 }));
    let position = storage.get(&entity);
    assert!(position.is_none());
}

#[test]
fn integration_query_component() {
    let mut storage = ecs::Storage::<Position>::new();
    let entity_1 = Entity::new(String::from("hello"));
    let entity_2 = Entity::new(String::from("goodbye"));
    storage.insert(&entity_1, Position { x: 2.0, y: 3.0 });
    storage.insert(&entity_2, Position { x: 1.0, y: 2.0 });

    assert!(storage.iter().count() == 2);

    assert!(
        storage
            .iter()
            .any(|data| data.0 == &entity_1 && data.1 == &Position { x: 2.0, y: 3.0 })
    );

    assert!(
        storage
            .iter()
            .any(|data| data.0 == &entity_2 && data.1 == &Position { x: 1.0, y: 2.0 })
    );
}

#[test]
fn test_mutable_storage_iterator() {
    let mut storage = ecs::Storage::<Position>::new();
    let entity_1 = Entity::new(String::from("hello"));
    let entity_2 = Entity::new(String::from("goodbye"));
    storage.insert(&entity_1, Position { x: 1.0, y: 2.0 });
    storage.insert(&entity_2, Position { x: 3.0, y: 4.0 });

    assert!(storage.iter_mut().count() == 2);

    {
        if let Some((_, component)) = storage.iter_mut().find(|data| data.0 == &entity_1) {
            component.x = 4f32;
            component.y = 5f32;
        }
    }
    {
        if let Some((_, component)) = storage.iter_mut().find(|data| data.0 == &entity_2) {
            component.x = 8f32;
            component.y = 9f32;
        }
    }

    assert_eq!(
        storage.get(&entity_1).expect("component not found"),
        &Position { x: 4.0, y: 5.0 }
    );
    assert_eq!(
        storage.get(&entity_2).expect("component not found"),
        &Position { x: 8.0, y: 9.0 }
    );
}
