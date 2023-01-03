use juniper::{graphql_object, FieldResult, GraphQLObject};
use sea_orm::prelude::Uuid;

use super::QueryRoot;
use crate::graphql::schema::Context;
use entity::{
    prelude::User,
    sea_orm_active_enums::{Role, Status},
    user,
};

#[derive(GraphQLObject, Debug)]
pub struct UserResponse {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: Role,
    pub status: Status,
}

impl UserResponse {
    pub fn single(model: &user::Model) -> Self {
        UserResponse {
            id: model.id.to_string(),
            name: model.name.to_string(),
            email: model.email.to_string(),
            role: model.role.to_owned(),
            status: model.status.to_owned(),
        }
    }

    pub fn multiple(models: Vec<user::Model>) -> Vec<Self> {
        models
            .into_iter()
            .map(|model| UserResponse::single(&model))
            .collect()
    }
}

#[graphql_object(Context = Context)]
impl QueryRoot {
    pub async fn user_by_email(ctx: &Context, email: String) -> FieldResult<Option<UserResponse>> {
        find_user_by_email(ctx, email).await
    }

    pub async fn user_by_id(ctx: &Context, id: String) -> FieldResult<Option<UserResponse>> {
        find_user_by_id(ctx, id).await
    }

    pub async fn users(ctx: &Context) -> FieldResult<Vec<UserResponse>> {
        get_users(ctx).await
    }
}

pub async fn find_user_by_email(ctx: &Context, email: String) -> FieldResult<Option<UserResponse>> {
    let conn = ctx.connection.as_ref();
    let found_user = User::find_one_by_email(&email, conn).await?;

    let res = found_user.map(|model| UserResponse::single(&model));
    Ok(res)
}

pub async fn find_user_by_id(ctx: &Context, id: String) -> FieldResult<Option<UserResponse>> {
    let conn = ctx.connection.as_ref();
    let id = Uuid::parse_str(&id)?;
    let found_user = User::find_one_by_id(&id, conn).await?;
    let res = found_user.map(|model| UserResponse::single(&model));
    Ok(res)
}

pub async fn get_users(ctx: &Context) -> FieldResult<Vec<UserResponse>> {
    let conn = ctx.connection.as_ref();
    let users = User::find_all(conn).await?;
    let res = UserResponse::multiple(users);
    Ok(res)
}
