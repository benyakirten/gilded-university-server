use migration::DbErr;
use sea_orm::{prelude::Uuid, EntityTrait, InsertResult, Set};
use serde::{Deserialize, Serialize};

use super::GQLResponse;
use entity::{
    prelude::User,
    sea_orm_active_enums::{Role, Status},
    user,
};
use gilded_university_server::connect_to_database;

pub mod user_integration;
pub mod user_mutation;
pub mod user_query;

pub async fn seed_users() -> Result<InsertResult<user::ActiveModel>, DbErr> {
    let conn = connect_to_database("TEST_DATABASE_URL").await.unwrap();
    let model_one = user::ActiveModel {
        id: Set(Uuid::new_v4()),
        email: Set("test@test.com".to_string()),
        name: Set("test user".to_string()),
        password: Set("testpassword".to_string()),
        status: Set(Status::Online),
        role: Set(Role::Guest),
    };
    let model_two = user::ActiveModel {
        id: Set(Uuid::new_v4()),
        email: Set("test2@test.com".to_string()),
        name: Set("test user2".to_string()),
        password: Set("testpassword".to_string()),
        status: Set(Status::Offline),
        role: Set(Role::Teacher),
    };
    User::insert_many(vec![model_one, model_two])
        .exec(&conn)
        .await
}

#[allow(dead_code)]
pub type GQLUsersRes = GQLResponse<GQLUsers>;
#[allow(dead_code)]
pub type GQLUserByEmailRes = GQLResponse<GQLUserByEmailResponse>;
#[allow(dead_code)]
pub type GQLUserByIdRes = GQLResponse<GQLUserByIdResponse>;

#[derive(Serialize, Deserialize, Debug)]
pub struct GQLUsers {
    pub users: Vec<GQLUserModel>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GQLUserByEmailResponse {
    #[serde(rename = "userByEmail")]
    pub user_by_email: Option<GQLUserModel>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GQLUserByIdResponse {
    #[serde(rename = "userById")]
    pub user_by_id: Option<GQLUserModel>,
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
