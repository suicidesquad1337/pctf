use async_graphql::{Enum, Object, ID};
use uuid::Uuid;

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// A ctf challenge
pub struct Challenge {
    /// The unique identifier of this session used as the primary key
    id: Uuid,
}

#[Object]
impl Challenge {
    pub async fn id(&self) -> ID {
        self.id.into()
    }

    /// Returns the type of the challenge
    // `type` is a reserved key word so raw identifiers are required. If you
    // want, you could also call this function something like `typ` and use
    // `graphql(name = "type")` to rename it in the graphql spec
    pub async fn r#type(&self) -> ChallengeType {
        ChallengeType::Reversing
    }
}

impl From<Uuid> for Challenge {
    fn from(id: Uuid) -> Self {
        Self { id }
    }
}

#[non_exhaustive]
#[derive(Enum, Clone, Copy, PartialEq, Eq, sqlx::Type, Debug)]
/// The type of a ctf [`Challenge`]
pub enum ChallengeType {
    Pwn,
    Web,
    Crypto,
    Reversing,
}
