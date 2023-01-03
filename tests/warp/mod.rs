use serde::{Deserialize, Serialize};

pub mod user;

#[derive(Serialize, Deserialize, Debug)]
pub struct GQLRequest<T> {
    query: String,
    variables: T,
}
