#[cfg(test)]
mod user_response {
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
        let ids: Vec<Uuid> = (0..9).map(|_| Uuid::new_v4()).collect();
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
