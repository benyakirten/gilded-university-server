#[cfg(test)]
mod user_response {
    use crate::graphql::query::user::UserResponse;
    use entity::{
        sea_orm_active_enums::{Role, Status},
        user as user_entity,
    };
    use sea_orm::prelude::Uuid;

    #[test]
    fn single_create_model() {
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
}
