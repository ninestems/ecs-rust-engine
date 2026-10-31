use ecs_rust_engine as ecs;

#[test]
fn integration_test_can_import_crate() {
    let value = ecs::some_fn();

    assert_eq!(value, "expected value");
}
