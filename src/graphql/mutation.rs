use std::io::ErrorKind;

use juniper::{graphql_object, FieldResult, GraphQLObject};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set};

use crate::{
    auth::{hash::hash, jwt::create_jwt},
    graphql::schema::Context,
};
use entity::{
    prelude::User,
    sea_orm_active_enums::{Role, Status},
    user,
};

#[derive(GraphQLObject)]
pub struct AuthResponse {
    pass: String,
    token: String,
}

impl AuthResponse {
    fn new(pass: &str, token: &str) -> Self {
        AuthResponse {
            pass: pass.to_string(),
            token: token.to_string(),
        }
    }
}

#[derive(GraphQLObject)]
pub struct SignoutResponse {
    success: bool,
}

impl SignoutResponse {
    fn complete() -> Self {
        SignoutResponse { success: true }
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
        let conn = ctx.connection.as_ref();
        let existing = User::find()
            .filter(user::Column::Email.eq(email.clone()))
            .one(conn)
            .await?;
        if existing.is_some() {
            return Err(ErrorKind::AlreadyExists.into());
        }

        let pass = hash(&password)?;
        let new_user = User::create_active_model(&email, &name, &pass);
        let res = user::Entity::insert(new_user).exec(conn).await?;

        let id = res.last_insert_id;
        let token = create_jwt(&id, &Role::Guest)?;
        Ok(AuthResponse::new(&pass, &token))
    }

    async fn signin(ctx: &Context, email: String, password: String) -> FieldResult<AuthResponse> {
        let conn = ctx.connection.as_ref();
        let found = User::find()
            .filter(user::Column::Email.eq(email))
            .one(conn)
            .await?;

        match found {
            Some(found) => {
                let password = hash(&password)?;
                if found.password != password {
                    return Err(ErrorKind::ConnectionRefused.into());
                }

                let mut found: user::ActiveModel = found.into();
                found.status = Set(Status::Online.to_owned());
                let found: user::Model = User::update(found).exec(conn).await?;

                let token = create_jwt(&found.id, &found.role)?;
                Ok(AuthResponse::new(&found.password, &token))
            }
            None => Err(ErrorKind::NotFound.into()),
        }
    }

    async fn signout(ctx: &Context, email: String) -> FieldResult<SignoutResponse> {
        let conn = ctx.connection.as_ref();
        let found = User::find()
            .filter(user::Column::Email.eq(email))
            .one(conn)
            .await?;

        match found {
            Some(found) => {
                let mut found: user::ActiveModel = found.into();
                found.status = Set(Status::Offline.to_owned());
                User::update(found).exec(conn).await?;

                Ok(SignoutResponse::complete())
            }
            None => Err(ErrorKind::NotFound.into()),
        }
    }
}
