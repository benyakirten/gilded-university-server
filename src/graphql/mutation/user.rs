use juniper::{graphql_object, FieldResult, GraphQLObject};
use sea_orm::Set;

use super::MutationRoot;
use crate::{
    auth::{
        hash::{hash, verify},
        jwt::create_jwt,
    },
    errors::UserError,
    graphql::schema::Context,
};
use entity::{
    prelude::User,
    sea_orm_active_enums::{Role, Status},
    user,
};

#[derive(GraphQLObject)]
pub struct AuthResponse {
    // TODO: Add refresh token
    pub token: String,
}

impl AuthResponse {
    pub fn new(token: &str) -> Self {
        AuthResponse {
            token: token.to_string(),
        }
    }
}

#[derive(GraphQLObject)]
pub struct SignoutResponse {
    pub success: bool,
}

impl SignoutResponse {
    pub fn complete() -> Self {
        SignoutResponse { success: true }
    }
}

#[graphql_object(Context = Context)]
impl MutationRoot {
    pub async fn signup(
        ctx: &Context,
        email: String,
        name: String,
        password: String,
    ) -> FieldResult<AuthResponse> {
        signup(ctx, email, name, password).await
    }

    pub async fn signin(
        ctx: &Context,
        email: String,
        password: String,
    ) -> FieldResult<AuthResponse> {
        signin(ctx, email, password).await
    }

    pub async fn signout(ctx: &Context, email: String) -> FieldResult<SignoutResponse> {
        signout(ctx, email).await
    }
}

// Any function that inserts/updates the database won't work with the mock database
pub async fn signup(
    ctx: &Context,
    email: String,
    name: String,
    password: String,
) -> FieldResult<AuthResponse> {
    let conn = ctx.connection.as_ref();
    let existing = User::find_one_by_email(&email, conn).await?;
    if existing.is_some() {
        return Err(UserError::UnableToComplete.into());
    }

    let pass = hash(&password)?;
    let new_user = User::create_active_model(&email, &name, &pass);
    let res = User::insert_one(new_user, conn).await?;

    let id = res.last_insert_id;
    let token = create_jwt(&id, &Role::Guest)?;
    Ok(AuthResponse::new(&token))
}

pub async fn signin(ctx: &Context, email: String, password: String) -> FieldResult<AuthResponse> {
    let conn = ctx.connection.as_ref();
    let found = User::find_one_by_email(&email, conn).await?;
    match found {
        Some(found) => {
            verify(&password, &found.password).map_err(|_| UserError::IncorrectEmailOrPassword)?;

            let mut found: user::ActiveModel = found.into();
            found.status = Set(Status::Online.to_owned());
            let found: user::Model = User::update_one(found, conn).await?;

            let token = create_jwt(&found.id, &found.role)?;
            Ok(AuthResponse::new(&token))
        }
        None => Err(UserError::IncorrectEmailOrPassword.into()),
    }
}

pub async fn signout(ctx: &Context, email: String) -> FieldResult<SignoutResponse> {
    let conn = ctx.connection.as_ref();
    let found = User::find_one_by_email(&email, conn).await?;

    // TODO: Verify token is from user that is trying to log out

    match found {
        Some(found) => {
            let mut found: user::ActiveModel = found.into();
            found.status = Set(Status::Offline.to_owned());
            User::update_one(found, conn).await?;

            Ok(SignoutResponse::complete())
        }
        None => Err(UserError::UnableToComplete.into()),
    }
}
