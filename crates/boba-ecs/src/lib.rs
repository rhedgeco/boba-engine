pub mod component;
pub mod entity;

pub use component::{Component, ComponentId};
pub use entity::Entity;

// re-export macros
pub use boba_ecs_macros::Component;
// place self in extern prelude so re-exports of this crate work with derive macro
extern crate self as boba_ecs;
