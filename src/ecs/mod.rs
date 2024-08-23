mod component;
mod config;
mod entity;
mod singleton;
mod system;

pub mod ecs {
    pub use super::component::*;
    pub use super::config::*;
    pub use super::entity::*;
    pub use super::singleton::*;
    pub use super::system::*;
}
