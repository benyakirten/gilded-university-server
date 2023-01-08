#[cfg(test)]
mod test_auth_response {
    use entity::sea_orm_active_enums::{Role, Status};

    use crate::graphql::{mutation::user::AuthResponse, user::GQLUser};

    #[test]
    fn create_new_auth_response() {
        let user = GQLUser {
            id: "id".to_string(),
            email: "test@test.com".to_string(),
            name: "test user".to_string(),
            role: Role::Teacher,
            status: Status::Offline,
        };
        let new = AuthResponse::new("token", user);
        assert_eq!(new.token, "token");
        assert_eq!(new.user.id, "id");
        assert_eq!(new.user.email, "test@test.com");
        assert_eq!(new.user.name, "test user");
        assert_eq!(new.user.role, Role::Teacher);
        assert_eq!(new.user.status, Status::Offline);
    }
}

#[cfg(test)]
mod test_signout_response {
    use crate::graphql::mutation::user::SignoutResponse;

    #[test]
    fn complete_signout_response() {
        let signout = SignoutResponse::complete();
        assert!(signout.success);
    }
}
