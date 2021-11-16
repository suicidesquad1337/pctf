//! Defines the node interface. This trait should be implemented by all types
//! with an unique identifier

use std::str::FromStr;

use async_graphql::{dataloader::DataLoader, Context, Interface, Object, Result, ID};
use uuid::Uuid;

use crate::{challenge::Challenge, loaders::ChallengeLoaderByID};

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
    Challenge(Challenge),
}

#[non_exhaustive]
#[derive(Default, Debug)]
/// The `node` query object
pub struct NodeQuery;

#[Object]
impl NodeQuery {
    // get a node with `id`. If the `Node` does not exist, it returns `null`.
    async fn node(&self, ctx: &Context<'_>, id: ID) -> Result<Option<Node>> {
        // currently, only `Challenge` is a `Node`, so we can directly query for
        // that
        let challenge: Option<_> = ctx
            .data_unchecked::<DataLoader<ChallengeLoaderByID>>()
            .load_one(Uuid::from_str(&id.0)?)
            .await?;

        // cant use into on Option even tho Node implements From<Challenge>
        Ok(challenge.map(|c| c.into()))
    }
}
