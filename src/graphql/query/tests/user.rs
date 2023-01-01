mod user {
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
    mod test_user_query {
        use sea_orm::prelude::Uuid;

        use crate::{graphql::query::user::*, testutils::create_mock_context};
        use entity::{
            sea_orm_active_enums::{Role, Status},
            user as user_entity,
        };

        #[tokio::test]
        async fn one_user_by_email() {
            let ids = (0..2).map(|_| Uuid::new_v4()).collect::<Vec<Uuid>>();
            let users: Vec<Vec<user_entity::Model>> = vec![vec![
                user_entity::Model {
                    id: ids[0].clone(),
                    email: "test1@test.com".to_string(),
                    name: "test user1".to_string(),
                    password: "testpass".to_string(),
                    status: Status::Online,
                    role: Role::Teacher,
                },
                user_entity::Model {
                    id: ids[1].clone(),
                    email: "test2@test.com".to_string(),
                    name: "test user2".to_string(),
                    password: "testpass".to_string(),
                    status: Status::Offline,
                    role: Role::Student,
                },
            ]];
            let context = create_mock_context(users, None);

            let got = user_by_email(&context, "test2@test.com".to_string())
                .await
                .unwrap()
                .unwrap();
            for &id in ids.iter() {
                println!("{}", id.to_string());
            }
            assert_eq!(got.id, ids[1].to_string());
            assert_eq!(got.email, "test2@test.com");
            assert_eq!(got.name, "test user2");
            assert_eq!(got.status, Status::Offline);
            assert_eq!(got.role, Role::Student);
        }
    }
}
