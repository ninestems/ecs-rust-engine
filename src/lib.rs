mod bundle;
mod entity;
mod storage;
mod world;

pub use entity::Entity;
pub use storage::Storage;
pub use world::World;

pub fn some_fn() -> String {
    String::from("expected value")
}
