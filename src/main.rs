use std::sync::Arc;

use dotenvy::dotenv;
use warp::{hyper::Uri, Filter};

use gilded_university_server::{
    connect_to_database,
    graphql::schema::{create_schema, Context},
};

#[tokio::main]
async fn main() {
    dotenv().expect(".env environment file not found");
    let redirect = warp::path::end().map(|| warp::redirect(Uri::from_static("/graphiql")));

    let connection = connect_to_database("DATABASE_URL")
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
                        token = _token;
                    }
                }
            }
            Context {
                connection: connection.clone(),
                token,
            }
        });
    let graphql_filter = juniper_warp::make_graphql_filter(create_schema(), state.boxed());

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["*"])
        .allow_methods(vec!["POST", "OPTIONS", "GET", "DELETE"]);
    warp::serve(
        warp::get()
            .and(warp::path("graphiql"))
            .and(juniper_warp::graphiql_filter("/graphql", None))
            .or(redirect)
            .or(warp::path("graphql").and(graphql_filter).with(cors)),
    )
    .run(([127, 0, 0, 1], 8080))
    .await
}
