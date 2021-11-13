use async_graphql::{EmptySubscription, MergedObject};

/// A type alias that uses our merged query/mutation objects
pub type Schema = async_graphql::Schema<Queries, Mutations, EmptySubscription>;

#[derive(MergedObject, Default)]
pub struct Queries();

#[derive(MergedObject, Default)]
pub struct Mutations();
