use ecs_rust_engine as ecs;
use std::collections::HashMap;

#[test]
fn integration_test_entity_equal_id() {
    let entity_1 = ecs::Entity::new(String::from("equal_id"));
    let entity_2 = ecs::Entity::new(String::from("equal_id"));
    assert_eq!(entity_1, entity_2);
}

#[test]
fn integration_test_entity_non_equal_id() {
    let entity_1 = ecs::Entity::new(String::from("equal_id"));
    let entity_2 = ecs::Entity::new(String::from("another_id"));
    assert_ne!(entity_1, entity_2);
}

#[test]
fn integration_test_entity_key_hashmap() {
    let mut entity_hashmap: HashMap<ecs::Entity, String> = HashMap::new();
    let entity_1 = ecs::Entity::new(String::from("equal_id"));
    entity_hashmap.insert(entity_1.clone(), String::from("value"));
    let value = entity_hashmap.get(&entity_1).expect("value not present");

    assert_eq!(value, "value");
}
