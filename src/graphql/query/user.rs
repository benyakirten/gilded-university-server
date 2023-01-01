use juniper::{graphql_object, FieldResult, GraphQLObject};
use sea_orm::{prelude::Uuid, ColumnTrait, EntityTrait, QueryFilter};

use super::QueryRoot;
use crate::graphql::schema::Context;
use entity::{
    prelude::User,
    sea_orm_active_enums::{Role, Status},
    user,
};

#[derive(GraphQLObject)]

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
    async fn userByEmail(ctx: &Context, email: String) -> FieldResult<Option<UserResponse>> {
        let conn = ctx.connection.as_ref();
        let found_user = User::find()
            .filter(user::Column::Email.eq(email))
            .one(conn)
            .await?;

        let res = found_user.map(|model| UserResponse::single(&model));
        Ok(res)
    }

    async fn userById(ctx: &Context, id: String) -> FieldResult<Option<UserResponse>> {
        let conn = ctx.connection.as_ref();
        let id = Uuid::parse_str(&id)?;
        let found_user = User::find_by_id(id).one(conn).await?;
        let res = found_user.map(|model| UserResponse::single(&model));
        Ok(res)
    }

    async fn users(ctx: &Context) -> FieldResult<Vec<UserResponse>> {
        let conn = ctx.connection.as_ref();
        let users = User::find().all(conn).await?;
        let res = UserResponse::multiple(users);
        Ok(res)
    }
}
