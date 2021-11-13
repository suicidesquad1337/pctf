use async_graphql::{dataloader::DataLoader as DL, Context, Object, Result, ID};

use super::Challenge;
use crate::loaders::ChallengeLoaderByName;

#[non_exhaustive]
#[derive(Debug, Default)]
/// GraphQL queries for a [`super::Challenge`]
pub struct ChallengeQueries;

#[Object]
impl ChallengeQueries {
    /// return a challenge by it's name. Usually you should not use this
    /// function, because minimal changes in the name like a newline would
    /// mean, that this query returns none.
    /// Instead, use the `Node` implementation of `Challenge` with the id.
    async fn challenge(&self, ctx: &Context<'_>, name: String) -> Result<Option<Challenge>> {
        Ok(ctx
            .data_unchecked::<DL<ChallengeLoaderByName>>()
            .load_one(name)
            .await?)
    }
}

#[non_exhaustive]
#[derive(Debug, Default)]
pub struct ChallengeMutations;

#[Object]
impl ChallengeMutations {
    async fn submit_flag(&self, _flag: String, _challenge: ID) -> Option<String> {
        todo!("send flag to supervisor")
    }
}
