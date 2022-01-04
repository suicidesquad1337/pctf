use std::ops::{Deref, DerefMut};

use crate::loaders::{
    ChallengeActiveLoaderByID, ChallengeCreatedAtLoaderByID, ChallengeHintsLoaderByID,
    ChallengeLongDescriptionLoaderByID as LongDescriptionLoaderByID, ChallengeNameLoaderByID,
    ChallengeShortDescriptionLoaderByID as ShortDescriptionLoaderByID, ChallengeTypeLoaderByID,
};
use async_graphql::{dataloader::DataLoader as DL, Context, Enum, Object, Result, ID};
use chrono::{DateTime, Utc};
use uuid::Uuid;

mod queries;

#[doc(inline)]
pub use queries::*;

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// A ctf challenge
pub struct Challenge {
    /// The unique identifier of this challenge used as the primary key
    id: Uuid,
}

#[Object(cache_control(max_age = 300))]
impl Challenge {
    pub async fn id(&self) -> ID {
        self.id.into()
    }

    /// The name of the challenge
    pub async fn name(&self, ctx: &Context<'_>) -> Result<String> {
        Ok(ctx
            .data_unchecked::<DL<ChallengeNameLoaderByID>>()
            .load_one(self.id)
            .await?
            .unwrap())
    }

    /// A short description for the challenge
    pub async fn short_description(&self, ctx: &Context<'_>) -> Result<Option<String>> {
        Ok(ctx
            .data_unchecked::<DL<ShortDescriptionLoaderByID>>()
            .load_one(self.id)
            .await?)
    }

    /// A long(er) description for the challenge
    pub async fn description(&self, ctx: &Context<'_>) -> Result<Option<String>> {
        Ok(ctx
            .data_unchecked::<DL<LongDescriptionLoaderByID>>()
            .load_one(self.id)
            .await?)
    }

    /// Hints that may help/spoiler the challenge
    pub async fn hints(&self, ctx: &Context<'_>) -> Result<Option<String>> {
        Ok(ctx
            .data_unchecked::<DL<ChallengeHintsLoaderByID>>()
            .load_one(self.id)
            .await?)
    }
    /// If the challenge is currently playable (e.g. if the challenge server
    /// is online or not)
    pub async fn is_active(&self, ctx: &Context<'_>) -> Result<bool> {
        Ok(ctx
            .data_unchecked::<DL<ChallengeActiveLoaderByID>>()
            .load_one(self.id)
            .await?
            .unwrap())
    }

    /// The date and time the challenge was published
    pub async fn created_at(&self, ctx: &Context<'_>) -> Result<DateTime<Utc>> {
        Ok(ctx
            .data_unchecked::<DL<ChallengeCreatedAtLoaderByID>>()
            .load_one(self.id)
            .await?
            .unwrap())
    }

    /// The type of the challenge
    // `type` is a reserved key word so raw identifiers are required. If you
    // want, you could also call this function something like `typ` and use
    // `graphql(name = "type")` to rename it in the graphql spec
    pub async fn r#type(&self, ctx: &Context<'_>) -> Result<ChallengeType> {
        Ok(ctx
            .data_unchecked::<DL<ChallengeTypeLoaderByID>>()
            .load_one(self.id)
            .await?
            .unwrap())
    }
}

impl From<Uuid> for Challenge {
    fn from(id: Uuid) -> Self {
        Self { id }
    }
}

impl Deref for Challenge {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target {
        &self.id
    }
}

impl DerefMut for Challenge {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.id
    }
}
#[non_exhaustive]
#[derive(Enum, Clone, Copy, PartialEq, Eq, sqlx::Type, Debug)]
#[sqlx(rename_all = "snake_case", type_name = "challenge_type")]
/// The type of a ctf [`Challenge`]
pub enum ChallengeType {
    Pwn,
    Web,
    Crypto,
    Reversing,
}
