use migration::DbErr;
use sea_orm::{DeleteResult, EntityTrait};
use serde::{Deserialize, Serialize};

use super::GQLResponse;
use entity::{prelude::User, user};
use gilded_university_server::connect_to_database;

pub mod user_integration;
pub mod user_mutation;
pub mod user_query;

pub async fn delete_all_users() -> Result<DeleteResult, DbErr> {
    let conn = connect_to_database("TEST_DATABASE_URL").await.unwrap();
    user::Entity::delete_many().exec(&conn).await
}

pub async fn get_all_users() -> Result<Vec<user::Model>, DbErr> {
    let conn = connect_to_database("TEST_DATABASE_URL").await.unwrap();
    User::find().all(&conn).await
}

#[allow(dead_code)]
pub type GQLUsersRes = GQLResponse<GQLUsers>;
#[allow(dead_code)]
pub type GQLUserRes = GQLResponse<GQLUser>;

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

#[allow(dead_code)]
type GQLSignupRes = GQLResponse<GQLSignupResponse>;
#[allow(dead_code)]
type GQLSigninRes = GQLResponse<GQLSigninResponse>;
#[allow(dead_code)]
type GQLSignoutRes = GQLResponse<GQLSignoutResponse>;

#[derive(Serialize, Deserialize, Debug)]
pub struct GQLSigninResponse {
    pub signin: GQLAuthResponse,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GQLSignupResponse {
    pub signup: GQLAuthResponse,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GQLAuthResponse {
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GQLSignoutResponse {
    pub signout: GQLSuccessResponse,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GQLSuccessResponse {
    pub success: bool,
}
