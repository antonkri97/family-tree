pub mod person;
pub mod schema;

use neo4j::{Error, Graph};

pub use person::Person;
pub use schema::init_schema;
