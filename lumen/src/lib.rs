// TODO: Well structured public API

mod environment;
pub use environment::Environment;

pub mod address;

pub mod dmx;

pub mod fixture;

pub mod fixture_set;

pub mod parameter;

pub mod value;

pub mod patch;
pub use patch::Patch;

pub mod universe;

mod query;
pub use query::query_builder::QueryBuilder;
