use migration::DbErr;
use sea_orm::{DatabaseConnection, DeleteResult, EntityTrait};
use warp::{filters::BoxedFilter, http::Response};

use entity::{prelude::User, user};
use gilded_university_server::{connect_to_database, create_gql_filter};

pub async fn make_graphql_filter() -> BoxedFilter<(Response<Vec<u8>>,)> {
    let connection = connect_to_test_database().await;
    user::Entity::delete_many().exec(&connection).await.unwrap();
    create_gql_filter(connection)
}

pub async fn delete_records(conn: &DatabaseConnection) -> Result<(), DbErr> {
    user::Entity::delete_many().exec(conn).await?;
    Ok(())
}

pub async fn connect_to_test_database() -> DatabaseConnection {
    let conn = connect_to_database("TEST_DATABASE_URL").await.unwrap();
    delete_records(&conn).await.unwrap();
    conn
}

pub async fn delete_all_users() -> Result<DeleteResult, DbErr> {
    let conn = connect_to_database("TEST_DATABASE_URL").await.unwrap();
    user::Entity::delete_many().exec(&conn).await
}

pub async fn get_all_users() -> Result<Vec<user::Model>, DbErr> {
    let conn = connect_to_database("TEST_DATABASE_URL").await.unwrap();
    User::find().all(&conn).await
}
