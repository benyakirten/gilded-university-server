#[cfg(test)]
mod test_auth_response {
    use crate::graphql::mutation::user::AuthResponse;

    #[test]
    fn create_new_auth_response() {
        let new = AuthResponse::new("token");
        assert_eq!(new.token, "token");
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
