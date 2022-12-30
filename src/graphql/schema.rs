use std::sync::Arc;

use juniper::{EmptySubscription, RootNode};
use sea_orm::DatabaseConnection;

use super::{mutation::MutationRoot, query::QueryRoot};

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub struct Context {
    pub connection: Arc<DatabaseConnection>,
}

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot, EmptySubscription::<Context>::new())
}

impl juniper::Context for Context {}
