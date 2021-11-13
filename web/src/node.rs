//! Defines the node interface. This trait should be implemented by all types
//! with an unique identifier

use async_graphql::{Interface, ID};

use crate::challenge::Challenge;

// interfaces in graphql are like traits in rust with downcasting, however,
// async_graphql does not support trait objects for interface. Instead, it uses
// an Enum and checks if all listed variants implement the required functions.
#[derive(Interface)]
#[graphql(field(name = "id", type = "ID"))]
/// A node is an interface that all objects with an ID implement.
/// It can be used for [global object identification][1].
///
/// [1]: https://graphql.org/learn/global-object-identification/
pub enum Node {
    Chalenge(Challenge),
}
