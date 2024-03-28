//! # sakura-ecs
//!
//! A ridiculously simple entity-component-system library because I have no idea what I'm doing.

/// A world is a container for entities and their components.
pub struct World {}

/// An entity is a unique identifier for a set of components.
pub struct Entity(usize);
