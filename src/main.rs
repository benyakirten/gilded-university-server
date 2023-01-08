use std::env;

use dotenvy::dotenv;
use warp::{http::Method, hyper::Uri, Filter};

use gilded_university_server::{connect_to_database, create_gql_filter};

#[tokio::main]
async fn main() {
    dotenv().expect(".env environment file not found");
    env::set_var("RUST_LOG", "warp_server");
    env_logger::init();

    let log = warp::log("warp_server");

    let redirect = warp::path::end().map(|| warp::redirect(Uri::from_static("/graphiql")));

    let connection = connect_to_database("DATABASE_URL")
        .await
        .expect("Unable to establish connection to database");

    println!("Connection established to database");

    let graphql_filter = create_gql_filter(connection);

    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(&[Method::GET, Method::POST, Method::DELETE])
        .allow_headers(vec![
            "Accept",
            "Accept-Encoding",
            "Accept-Language",
            "Access-Control-Request-Headers",
            "Access-Control-Request-Method",
            "Connection",
            "Host",
            "Origin",
            "Referer",
            "Sec-Fetch-Mode",
            "Sec-Fetch-Mode",
            "Sec-Fetch-Site",
            "User-Agent",
            "accept",
            "sec-ch-ua",
            "sec-ch-ua-mobile",
            "sec-ch-ua-platform",
            "Content-Length",
            "content-type",
        ])
        .allow_credentials(false);

    // TODO: Set host by environment variable
    println!("Starting host at localhost:8080");

    warp::serve(
        warp::get()
            .and(warp::path("graphiql"))
            .and(juniper_warp::graphiql_filter("/graphql", None))
            .or(redirect)
            .or(warp::path("graphql").and(graphql_filter))
            .with(cors)
            .with(log),
    )
    .run(([127, 0, 0, 1], 8080))
    .await
}
