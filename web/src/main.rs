use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription,
};
use async_graphql_rocket::{Request, Response};
use rocket::{response::content::Html, State};
use schema::{Mutations, Queries, Schema};

#[macro_use]
extern crate rocket;

pub mod schema;

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
fn rocket() -> _ {
    // generate the schema
    let schema = Schema::build(Queries(), Mutations(), EmptySubscription).finish();

    rocket::build()
        .manage(schema)
        .mount("/", routes![graphql_request, graphql_playground])
}
