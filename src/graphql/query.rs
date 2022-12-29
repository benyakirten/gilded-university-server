use juniper::{graphql_object, FieldResult, GraphQLObject};

use super::schema::Context;

pub struct QueryRoot;

#[derive(GraphQLObject)]
pub struct Test {
    name: String,
}

#[graphql_object(Context = Context)]
impl QueryRoot {
    async fn test(ctx: &Context) -> FieldResult<Test> {
        Ok(Test {
            name: "Test name".to_string(),
        })
    }
}
