use serde::{Deserialize, Serialize};

pub mod user_integration;
pub mod user_mutation;
pub mod user_query;

#[derive(Serialize, Deserialize, Debug)]
pub struct GQLUserResponse<T: Serialize> {
    pub data: T,
}

pub type GQLUsersRes = GQLUserResponse<GQLUsers>;
pub type GQLUserRes = GQLUserResponse<GQLUser>;

#[derive(Serialize, Deserialize, Debug)]
pub struct GQLUsers {
    pub users: Vec<GQLUser>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GQLUser {
    pub user: GQLUserModel,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GQLUserModel {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: String,
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GQLAuthResponse {
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GQLSignoutResponse {
    pub success: bool,
}
