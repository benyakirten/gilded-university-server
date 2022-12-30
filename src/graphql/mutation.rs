use juniper::{graphql_object, FieldResult, GraphQLObject};

use super::schema::Context;
use crate::auth::{hash::hash, jwt::create_jwt};

use ::entity::sea_orm_active_enums::Role;

#[derive(GraphQLObject)]
pub struct AuthResponse {
    pass: String,
    token: String,
}

impl AuthResponse {
    fn new(pass: &str, token: &str) -> AuthResponse {
        AuthResponse {
            pass: pass.to_string(),
            token: token.to_string(),
        }
    }
}

pub struct MutationRoot;

#[graphql_object(Context = Context)]
impl MutationRoot {
    async fn signup(
        ctx: &Context,
        email: String,
        name: String,
        password: String,
    ) -> FieldResult<AuthResponse> {
        // Add the user to the database, get specify role
        todo!();
        let pass = hash(&password).await?;
        let token = create_jwt("1", &Role::Guest).await?;
        Ok(AuthResponse::new(&pass, &token))
    }
}
