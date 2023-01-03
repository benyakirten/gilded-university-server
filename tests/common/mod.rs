use std::sync::Arc;

use warp::{filters::BoxedFilter, http::Response, Filter};

use gilded_university_server::{
    connect_to_database,
    graphql::schema::{create_schema, Context},
};

#[allow(dead_code)]
pub async fn make_graphql_filter() -> BoxedFilter<(Response<Vec<u8>>,)> {
    let connection = connect_to_database("TEST_DATABASE_URL").await.unwrap();
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
    juniper_warp::make_graphql_filter(create_schema(), state.boxed())
}
