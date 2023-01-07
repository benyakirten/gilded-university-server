use juniper::{graphql_object, FieldResult};
use sea_orm::prelude::Uuid;

use super::QueryRoot;
use crate::graphql::{schema::Context, user::GQLUser};
use entity::prelude::User;

#[graphql_object(Context = Context)]
impl QueryRoot {
    pub async fn user_by_email(ctx: &Context, email: String) -> FieldResult<Option<GQLUser>> {
        find_user_by_email(ctx, email).await
    }

    pub async fn user_by_id(ctx: &Context, id: String) -> FieldResult<Option<GQLUser>> {
        find_user_by_id(ctx, id).await
    }

    pub async fn users(ctx: &Context) -> FieldResult<Vec<GQLUser>> {
        get_users(ctx).await
    }
}

pub async fn find_user_by_email(ctx: &Context, email: String) -> FieldResult<Option<GQLUser>> {
    let conn = ctx.connection.as_ref();
    let found_user = User::find_one_by_email(&email, conn).await?;

    let res = found_user.map(|model| GQLUser::single(&model));
    Ok(res)
}

pub async fn find_user_by_id(ctx: &Context, id: String) -> FieldResult<Option<GQLUser>> {
    let conn = ctx.connection.as_ref();
    let id = Uuid::parse_str(&id)?;
    let found_user = User::find_one_by_id(&id, conn).await?;
    let res = found_user.map(|model| GQLUser::single(&model));
    Ok(res)
}

pub async fn get_users(ctx: &Context) -> FieldResult<Vec<GQLUser>> {
    let conn = ctx.connection.as_ref();
    let users = User::find_all(conn).await?;
    let res = GQLUser::multiple(users);
    Ok(res)
}
