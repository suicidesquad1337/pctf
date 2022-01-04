use std::sync::Arc;

use async_graphql::{
    dataloader::DataLoader as DL,
    extensions::ApolloTracing,
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription,
};
use async_graphql_rocket::{Request, Response};
use rocket::{figment::providers::Env, response::content::Html, Config as RocketConfig, State};
use schema::{Mutations, Queries, Schema};
use sqlx::PgPool;

#[macro_use]
extern crate rocket;

pub mod challenge;
mod config;
pub mod loaders;
pub mod node;
pub mod schema;
use loaders::*;

#[doc(inline)]
pub use config::Config;

#[post("/graphql", data = "<request>", format = "application/json")]
/// Route to accept incoming GraphQL requests via HTTP POST
async fn graphql_request(schema: &State<Schema>, request: Request) -> Response {
    request.execute(schema).await
}

#[get("/playground")]
/// Displays `GraphQL playground`, a tool to inspect and try out the GraphQL schema
fn graphql_playground() -> Html<String> {
    Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

#[launch]
async fn rocket() -> _ {
    // create config from Rocket.toml or env
    let config = RocketConfig::figment()
        // merge with prefixed env variables
        .merge(Env::prefixed("PCTF_"))
        .extract::<Config>()
        .expect("cannot read config");

    let db = PgPool::connect(&config.db_uri)
        .await
        .expect("cannot connect to database");
    // run database migrations
    sqlx::migrate!()
        .run(&db)
        .await
        .expect("cannot run database migrations");

    // the dataloader used by the the front loaders
    let s_l = Arc::new(DL::new(ChallengeLoader::new(db.clone())));
    // generate the schema
    let schema = Schema::build(Queries::default(), Mutations::default(), EmptySubscription)
        // register the different data loaders
        .data(DL::new(ChallengeLoaderByID::new(db.clone())))
        .data(DL::new(ChallengeNameLoaderByID::new(s_l.clone())))
        .data(DL::new(ChallengeTypeLoaderByID::new(s_l.clone())))
        .data(DL::new(ChallengeShortDescriptionLoaderByID::new(
            s_l.clone(),
        )))
        .data(DL::new(ChallengeLongDescriptionLoaderByID::new(
            s_l.clone(),
        )))
        .data(DL::new(ChallengeHintsLoaderByID::new(s_l.clone())))
        .data(DL::new(ChallengeCreatedAtLoaderByID::new(s_l.clone())))
        .data(DL::new(ChallengeActiveLoaderByID::new(s_l)))
        .data(db.clone());

    // enable tracing if wanted
    let schema = if config.tracing {
        schema.extension(ApolloTracing)
    } else {
        schema
    }
    .finish();

    rocket::build()
        .manage(schema)
        .manage(db)
        .mount("/", routes![graphql_request, graphql_playground])
}
