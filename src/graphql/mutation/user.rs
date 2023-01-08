use juniper::{graphql_object, FieldResult, GraphQLObject};
use sea_orm::Set;

use super::MutationRoot;
use crate::{
    auth::{
        hash::{hash, verify},
        jwt::{create_jwt, get_claims_from_token},
    },
    errors::{AuthorizationError, UserError},
    graphql::{schema::Context, user::GQLUser},
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
    pub user: GQLUser,
}

impl AuthResponse {
    pub fn new(token: &str, user: GQLUser) -> Self {
        AuthResponse {
            token: token.to_string(),
            user,
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
    let res = User::insert_one(new_user.clone(), conn).await?;

    let id = res.last_insert_id;
    let token = create_jwt(&id, &Role::Guest)?;

    let user = GQLUser::from_active_model(new_user);

    Ok(AuthResponse::new(&token, user))
}

pub async fn signin(ctx: &Context, email: String, password: String) -> FieldResult<AuthResponse> {
    let conn = ctx.connection.as_ref();
    let found = User::find_one_by_email(&email, conn).await?;
    match found {
        Some(found) => {
            if found.status != Status::Offline {
                return Err(UserError::UnableToComplete.into());
            }
            verify(&password, &found.password).map_err(|_| UserError::IncorrectEmailOrPassword)?;

            let mut found: user::ActiveModel = found.into();
            found.status = Set(Status::Online.to_owned());
            let found: user::Model = User::update_one(found, conn).await?;

            let token = create_jwt(&found.id, &found.role)?;
            let user = GQLUser::single(&found);
            Ok(AuthResponse::new(&token, user))
        }
        None => Err(UserError::IncorrectEmailOrPassword.into()),
    }
}

pub async fn signout(ctx: &Context, email: String) -> FieldResult<SignoutResponse> {
    if ctx.token.is_empty() {
        return Err(AuthorizationError::TokenMissing.into());
    }
    let conn = ctx.connection.as_ref();
    let found = User::find_one_by_email(&email, conn).await?;

    match found {
        Some(found) => {
            let claims = get_claims_from_token(&ctx.token)?;
            if found.id != claims.sub {
                return Err(UserError::UnableToComplete.into());
            }

            let mut found: user::ActiveModel = found.into();
            found.status = Set(Status::Offline.to_owned());
            User::update_one(found, conn).await?;

            Ok(SignoutResponse::complete())
        }
        None => Err(UserError::UnableToComplete.into()),
    }
}
