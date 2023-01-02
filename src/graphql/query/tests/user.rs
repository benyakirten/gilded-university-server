#[cfg(test)]
mod test_user_response {
    use crate::graphql::query::user::UserResponse;
    use entity::{
        sea_orm_active_enums::{Role, Status},
        user as user_entity,
    };
    use sea_orm::prelude::Uuid;

    #[test]
    fn create_single_model() {
        let id = Uuid::new_v4();
        let model = user_entity::Model {
            id,
            name: "test user".to_string(),
            email: "test@test.com".to_string(),
            password: "passwordhash".to_string(),
            role: Role::Student,
            status: Status::Hidden,
        };
        let got = UserResponse::single(&model);
        assert_eq!(got.email, "test@test.com");
        assert_eq!(got.id, id.to_string());
        assert_eq!(got.name, "test user");
        assert_eq!(got.role, Role::Student);
        assert_eq!(got.status, Status::Hidden);
    }

    #[test]
    fn create_multiple_models() {
        let ids = (0..9).map(|_| Uuid::new_v4()).collect::<Vec<Uuid>>();
        let roles = vec![Role::Guest, Role::Student, Role::Teacher, Role::Admin];
        let statuses = vec![Status::Online, Status::Offline, Status::Hidden];
        let models: Vec<user_entity::Model> = (0..9)
            .map(|i| user_entity::Model {
                id: ids[i].clone(),
                name: format!("test user{}", i),
                email: format!("test{}@test.com", i),
                password: "passwordhash".to_string(),
                role: roles[i % 4].clone(),
                status: statuses[i % 3].clone(),
            })
            .collect();
        let responses = UserResponse::multiple(models);

        for i in 0..9 {
            let got = &responses[i];
            assert_eq!(got.id, ids[i].to_string());
            assert_eq!(got.email, format!("test{}@test.com", i));
            assert_eq!(got.name, format!("test user{}", i));
            assert_eq!(got.role, roles[i % 4].clone());
            assert_eq!(got.status, statuses[i % 3].clone());
        }
    }
}

#[cfg(test)]
mod test_find_user_by_email {
    use std::vec;

    use migration::DbErr;
    use sea_orm::prelude::Uuid;

    use crate::{
        graphql::query::user::*,
        testutils::{create_errored_context, create_mock_context},
    };
    use entity::{
        sea_orm_active_enums::{Role, Status},
        user as user_entity,
    };

    #[tokio::test]
    async fn find_one_user_for_find_user_by_email() {
        let id = Uuid::new_v4();
        let users: Vec<Vec<user_entity::Model>> = vec![vec![user_entity::Model {
            id: id.clone(),
            email: "test1@test.com".to_string(),
            name: "test user1".to_string(),
            password: "testpass".to_string(),
            status: Status::Online,
            role: Role::Teacher,
        }]];
        let context = create_mock_context(users, None);

        let got = find_user_by_email(&context, "test@test.com".to_string())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(got.id, id.to_string());
        assert_eq!(got.email, "test1@test.com");
        assert_eq!(got.name, "test user1");
        assert_eq!(got.status, Status::Online);
        assert_eq!(got.role, Role::Teacher);
    }

    #[tokio::test]
    async fn find_no_user_for_find_user_by_email() {
        let results: Vec<Vec<user_entity::Model>> = vec![vec![]];
        let context = create_mock_context(results, None);

        let got = find_user_by_email(&context, "test@test.com".to_string())
            .await
            .unwrap();

        assert!(got.is_none());
    }

    #[tokio::test]
    async fn get_error_for_find_user_by_email() {
        let context = create_errored_context(vec![DbErr::ConnectionAcquire.into()], None);
        let got = find_user_by_email(&context, "test@test.com".to_string()).await;

        assert!(got.is_err());
    }
}

#[cfg(test)]
mod test_find_user_by_id {
    use std::vec;

    use migration::DbErr;
    use sea_orm::prelude::Uuid;

    use crate::{
        graphql::query::user::*,
        testutils::{create_errored_context, create_mock_context},
    };
    use entity::{
        sea_orm_active_enums::{Role, Status},
        user as user_entity,
    };

    #[tokio::test]
    async fn find_one_user_for_find_user_by_id() {
        let id = Uuid::new_v4();
        let users: Vec<Vec<user_entity::Model>> = vec![vec![user_entity::Model {
            id: id.clone(),
            email: "test1@test.com".to_string(),
            name: "test user1".to_string(),
            password: "testpass".to_string(),
            status: Status::Online,
            role: Role::Teacher,
        }]];
        let context = create_mock_context(users, None);

        let got = find_user_by_id(&context, id.to_string())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(got.id, id.to_string());
        assert_eq!(got.email, "test1@test.com");
        assert_eq!(got.name, "test user1");
        assert_eq!(got.status, Status::Online);
        assert_eq!(got.role, Role::Teacher);
    }

    #[tokio::test]
    async fn find_no_user_for_find_user_by_id() {
        let results: Vec<Vec<user_entity::Model>> = vec![vec![]];
        let context = create_mock_context(results, None);

        let got = find_user_by_id(&context, Uuid::new_v4().to_string())
            .await
            .unwrap();

        assert!(got.is_none());
    }

    #[tokio::test]
    async fn get_error_for_find_user_by_id() {
        let context = create_errored_context(vec![DbErr::ConnectionAcquire.into()], None);
        let got = find_user_by_id(&context, Uuid::new_v4().to_string()).await;

        assert!(got.is_err());
    }
}

#[cfg(test)]
mod test_get_users {
    use std::vec;

    use migration::DbErr;
    use sea_orm::prelude::Uuid;

    use crate::{
        graphql::query::user::*,
        testutils::{create_errored_context, create_mock_context},
    };
    use entity::{
        sea_orm_active_enums::{Role, Status},
        user as user_entity,
    };

    #[tokio::test]
    async fn find_users_for_get_users() {
        let ids = (0..10).map(|_| Uuid::new_v4()).collect::<Vec<Uuid>>();
        let roles = vec![Role::Guest, Role::Student, Role::Teacher, Role::Admin];
        let statuses = vec![Status::Online, Status::Offline, Status::Hidden];
        let users = (0..10)
            .map(|i| user_entity::Model {
                id: ids[i].clone(),
                email: format!("test{}@test.com", i),
                name: format!("test user{}", i),
                password: "testpass".to_string(),
                status: statuses[i % 3].clone(),
                role: roles[i % 4].clone(),
            })
            .collect::<Vec<user_entity::Model>>();
        let results = vec![users];
        let context = create_mock_context(results, None);

        let result = get_users(&context).await.unwrap();
        for i in 0..10 {
            let got = &result[i];
            assert_eq!(got.id, ids[i].to_string());
            assert_eq!(got.email, format!("test{}@test.com", i));
            assert_eq!(got.name, format!("test user{}", i));
            assert_eq!(got.role, roles[i % 4].clone());
            assert_eq!(got.status, statuses[i % 3].clone());
        }
    }

    #[tokio::test]
    async fn find_no_users_for_get_users() {
        let results: Vec<Vec<user_entity::Model>> = vec![vec![]];
        let context = create_mock_context(results, None);

        let result = get_users(&context).await.unwrap();
        assert_eq!(result.len(), 0);
    }

    #[tokio::test]
    async fn get_error_for_get_users() {
        let context = create_errored_context(vec![DbErr::ConnectionAcquire.into()], None);
        let got = get_users(&context).await;

        assert!(got.is_err());
    }
}
