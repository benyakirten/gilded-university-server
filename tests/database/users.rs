#[cfg(test)]
mod integration_database_user {
    use dotenvy::dotenv;
    use sea_orm::{prelude::Uuid, Set};

    use crate::common::{connect_to_test_database, delete_all_users, get_all_users};
    use entity::{
        prelude::User,
        sea_orm_active_enums::{Role, Status},
        user,
    };

    #[tokio::test]
    async fn user_test() {
        dotenv().ok();
        let conn = connect_to_test_database().await;

        let model_one = user::ActiveModel {
            id: sea_orm::ActiveValue::Set(Uuid::new_v4()),
            email: sea_orm::ActiveValue::Set("test@test.com".to_string()),
            name: sea_orm::ActiveValue::Set("test user".to_string()),
            password: sea_orm::ActiveValue::Set("testpassword".to_string()),
            status: sea_orm::ActiveValue::Set(Status::Online),
            role: sea_orm::ActiveValue::Set(Role::Guest),
        };
        let model_two = user::ActiveModel {
            id: sea_orm::ActiveValue::Set(Uuid::new_v4()),
            email: sea_orm::ActiveValue::Set("test2@test.com".to_string()),
            name: sea_orm::ActiveValue::Set("test user2".to_string()),
            password: sea_orm::ActiveValue::Set("testpassword".to_string()),
            status: sea_orm::ActiveValue::Set(Status::Offline),
            role: sea_orm::ActiveValue::Set(Role::Teacher),
        };

        User::insert_one(model_one, &conn).await.unwrap();
        User::insert_one(model_two, &conn).await.unwrap();

        let users = get_all_users().await.unwrap();

        assert_eq!(users[0].email, "test@test.com");
        assert_eq!(users[0].name, "test user");
        assert_eq!(users[0].role, Role::Guest);
        assert_eq!(users[0].status, Status::Online);

        assert_eq!(users[1].email, "test2@test.com");
        assert_eq!(users[1].name, "test user2");
        assert_eq!(users[1].role, Role::Teacher);
        assert_eq!(users[1].status, Status::Offline);

        let user2 = User::find_one_by_email("test2@test.com", &conn)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(user2.email, "test2@test.com");
        assert_eq!(user2.name, "test user2");
        assert_eq!(user2.role, Role::Teacher);
        assert_eq!(user2.status, Status::Offline);

        let mut mod_user: user::ActiveModel = user2.into();
        mod_user.status = Set(Status::Online.to_owned());

        let res = User::update_one(mod_user, &conn).await.unwrap();
        assert_eq!(res.email, "test2@test.com");
        assert_eq!(res.name, "test user2");
        assert_eq!(res.role, Role::Teacher);
        assert_eq!(res.status, Status::Online);

        let user_again = User::find_one_by_id(&res.id, &conn).await.unwrap().unwrap();
        assert_eq!(user_again.email, "test2@test.com");
        assert_eq!(user_again.name, "test user2");
        assert_eq!(user_again.role, Role::Teacher);
        assert_eq!(user_again.status, Status::Online);

        delete_all_users().await.unwrap();
    }
}
