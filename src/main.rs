use std::sync::Arc;

use dotenvy::dotenv;
use warp::{http::Response, Filter};

mod auth;
mod graphql;

use crate::graphql::schema::{create_schema, Context};
use gilded_university_server::connect_to_database;

#[tokio::main]
async fn main() {
    dotenv().expect(".env environment file not found");
    let homepage = warp::path::end().map(|| {
        Response::builder()
            .header("content-type", "text/html")
            .body(
                "<html><h1>juniper_warp</h1><div>visit <a href=\"/graphiql\">/graphiql</a></html>",
            )
    });

    let connection = connect_to_database()
        .await
        .expect("Unable to establish connection to database");
    let connection = Arc::new(connection);
    let state = warp::any()
        .and(warp::header::optional::<String>("Authorization"))
        .map(move |auth: Option<String>| -> Context {
            let mut token = "".to_string();
            if auth.is_some() {
                let iter = &mut auth.into_iter();
                if iter.next() == Some("Bearer".to_string()) {
                    if let Some(_token) = iter.next() {
                        token = _token.to_string();
                    }
                }
            }
            Context {
                connection: connection.clone(),
                token,
            }
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
