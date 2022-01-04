use std::{collections::HashMap, sync::Arc};

use async_graphql::dataloader::{DataLoader, Loader};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::challenge::{Challenge, ChallengeType};

use super::{ChallengeField, ChallengeFieldSelection, ChallengeLoader};

/// Macro so I dont have to repeat this seven times:
/*
pub struct ChallengeNameLoaderByID {
    loader: Arc<DataLoader<ChallengeLoader>>,
}

#[async_trait]
impl Loader<Uuid> for ChallengeNameLoaderByID {
    type Value = String;
    type Error = Arc<sqlx::Error>;
    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let keys = keys
            .iter()
            .map(|k| (Challenge::from(*k), ChallengeFieldSelection::Name));
        Ok(self
            .loader
            .load_many(keys)
            .await?
            .into_iter()
            .map(|((challenge, _), resp)| {
                let name = match resp {
                    ChallengeField::Name(name) => name,
                    // this will never trigger as long as ChallengeFieldSelection::Name
                    // is ChallengeField::Name, if its not, its a bug
                    _ => panic!("found invalid variant, this is a bug!"),
                };
                (*challenge, name)
            })
            .collect())
    }
}
*/
macro_rules! c_loader {
    ($name:ident, $val:ty, $requested:path, $expected:path) => {
        pub struct $name {
            loader: Arc<DataLoader<ChallengeLoader>>,
        }

        impl $name {
            #[deny(unused)]
            pub fn new(loader: Arc<DataLoader<ChallengeLoader>>) -> Self {
                Self { loader }
            }
        }

        #[async_trait]
        impl Loader<Uuid> for $name {
            type Value = $val;
            type Error = Arc<sqlx::Error>;

            async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
                let keys = keys.iter().map(|k| (Challenge::from(*k), $requested));
                Ok(self
                    .loader
                    .load_many(keys)
                    .await?
                    .into_iter()
                    .map(|((challenge, _), resp)| {
                        let value = match resp {
                            $expected(value) => value,
                            _ => panic!("found invalid variant, this is a bug!"),
                        };
                        (*challenge, value)
                    })
                    .collect())
            }
        }
    };
}

c_loader!(
    ChallengeNameLoaderByID,
    String,
    ChallengeFieldSelection::Name,
    ChallengeField::Name
);
c_loader!(
    ChallengeTypeLoaderByID,
    ChallengeType,
    ChallengeFieldSelection::Type,
    ChallengeField::Type
);
c_loader!(
    ChallengeShortDescriptionLoaderByID,
    String,
    ChallengeFieldSelection::ShortDescription,
    ChallengeField::ShortDescription
);
c_loader!(
    ChallengeLongDescriptionLoaderByID,
    String,
    ChallengeFieldSelection::LongDescription,
    ChallengeField::LongDescription
);
c_loader!(
    ChallengeHintsLoaderByID,
    String,
    ChallengeFieldSelection::Hints,
    ChallengeField::Hints
);
c_loader!(
    ChallengeCreatedAtLoaderByID,
    DateTime<Utc>,
    ChallengeFieldSelection::CreatedAt,
    ChallengeField::CreatedAt
);
c_loader!(
    ChallengeActiveLoaderByID,
    bool,
    ChallengeFieldSelection::Active,
    ChallengeField::Active
);
