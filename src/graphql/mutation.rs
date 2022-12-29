use juniper::{graphql_object, FieldResult, GraphQLObject};

use super::schema::Context;
use crate::{
    auth::{hash::hash, jwt::create_jwt},
    models::user::Role,
};

#[derive(GraphQLObject)]
pub struct AuthResponse {
    pass: String,
    token: String,
}

pub struct MutationRoot;

#[graphql_object(Context = Context)]
impl MutationRoot {
    async fn signup(ctx: &Context, email: String, password: String) -> FieldResult<AuthResponse> {
        let (pass, _) = hash(&password).await?;
        let token = create_jwt("1", &Role::Admin).await?;
        Ok(AuthResponse { pass, token })
    }
}
