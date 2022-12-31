use juniper::{graphql_object, FieldResult, GraphQLObject};
use sea_orm::{prelude::Uuid, ColumnTrait, EntityTrait, QueryFilter};

use super::schema::Context;
use entity::{prelude::User, user};

pub struct QueryRoot;

#[derive(GraphQLObject)]
pub struct UserResponse {
    name: String,
    email: String,
    role: String,
    status: String,
}

impl UserResponse {
    fn new(model: &user::Model) -> UserResponse {
        UserResponse {
            name: model.name.to_string(),
            email: model.email.to_string(),
            role: model.role.to_str(),
            status: model.status.to_str(),
        }
    }
}

#[graphql_object(Context = Context)]
impl QueryRoot {
    async fn userWithEmail(ctx: &Context, email: String) -> FieldResult<Option<UserResponse>> {
        let conn = ctx.connection.as_ref();
        let found_user = User::find()
            .filter(user::Column::Email.eq(email))
            .one(conn)
            .await?;
        let res = found_user.map(|model| UserResponse::new(&model));
        Ok(res)
    }

    async fn userById(ctx: &Context, id: String) -> FieldResult<Option<UserResponse>> {
        let conn = ctx.connection.as_ref();
        let id = Uuid::parse_str(&id)?;
        let found_user = User::find_by_id(id).one(conn).await?;
        let res = found_user.map(|model| UserResponse::new(&model));
        Ok(res)
    }
}
