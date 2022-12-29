use dotenvy::dotenv;
use gilded_university_server::connect_to_database;
use warp::{http::Response, Filter};

mod auth;
mod graphql;
mod models;

use crate::graphql::schema::{create_schema, Context};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let homepage = warp::path::end().map(|| {
        Response::builder()
            .header("content-type", "text/html")
            .body(
                "<html><h1>juniper_warp</h1><div>visit <a href=\"/graphiql\">/graphiql</a></html>",
            )
    });

    let connection = connect_to_database().await;
    let state = warp::any().map(move || Context {
        connection: connection.clone(),
    });
    let graphql_filter = juniper_warp::make_graphql_filter(create_schema(), state.boxed());

    warp::serve(
        warp::get()
            .and(warp::path("graphiql"))
            .and(juniper_warp::graphiql_filter("/graphql", None))
            .or(homepage)
            .or(warp::path("graphql").and(graphql_filter)),
    )
    .run(([127, 0, 0, 1], 8080))
    .await
}
