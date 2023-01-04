#[cfg(test)]
mod integration_warp_user_query {
    use dotenvy::dotenv;

    use crate::{
        common::make_graphql_filter,
        warp::{
            user::{delete_all_users, seed_users, GQLUserByEmailRes, GQLUserByIdRes, GQLUsersRes},
            GQLRequest,
        },
    };

    // To make sure the test steps perform exactly as needed
    // i.e. inserting/deleting records sequentially
    // We will use one function that will perform all the test
    #[tokio::test]
    async fn user_query() {
        dotenv().ok();
        let filter = make_graphql_filter().await;
        seed_users().await.unwrap();

        let body: GQLRequest<()> = GQLRequest {
            query: r#"
                query {
                    users {
                        id
                        email
                        name
                        role
                        status
                    }
            }"#
            .to_string(),
            variables: None,
        };
        let response = warp::test::request()
            .method("POST")
            .json(&body)
            .filter(&filter)
            .await
            .unwrap();

        let response_json: GQLUsersRes = serde_json::from_slice(response.body()).unwrap();
        assert!(response_json.data.is_some());
        assert!(response_json.errors.is_none());

        let data = response_json.data.unwrap();
        let user1 = &data.users[0];
        let user2 = &data.users[1];

        let id1 = data.users[0].id.clone();

        assert_eq!(user1.email, "test@test.com");
        assert_eq!(user1.name, "test user");
        assert_eq!(user1.role, "GUEST");
        assert_eq!(user1.status, "ONLINE");

        assert_eq!(user2.email, "test2@test.com");
        assert_eq!(user2.name, "test user2");
        assert_eq!(user2.role, "TEACHER");
        assert_eq!(user2.status, "OFFLINE");

        let body: GQLRequest<()> = GQLRequest {
            query: r#"
                query {
                    userByEmail(email: "test2@test.com") {
                        id
                        email
                        name
                        role
                        status
                    }
                }
            "#
            .to_string(),
            variables: None,
        };
        let response = warp::test::request()
            .method("POST")
            .json(&body)
            .filter(&filter)
            .await
            .unwrap();

        let response_json: GQLUserByEmailRes = serde_json::from_slice(response.body()).unwrap();
        assert!(response_json.data.is_some());
        assert!(response_json.errors.is_none());

        let user = response_json.data.unwrap().user_by_email;
        assert!(user.is_some());

        let user = user.unwrap();
        assert_eq!(user.email, "test2@test.com");
        assert_eq!(user.name, "test user2");
        assert_eq!(user.role, "TEACHER");
        assert_eq!(user.status, "OFFLINE");

        let body: GQLRequest<()> = GQLRequest {
            query: r#"
                query {
                    userByEmail(email: "notanemail") {
                        id
                        email
                        name
                        role
                        status
                    }
                }
            "#
            .to_string(),
            variables: None,
        };
        let response = warp::test::request()
            .method("POST")
            .json(&body)
            .filter(&filter)
            .await
            .unwrap();

        let response_json: GQLUserByEmailRes = serde_json::from_slice(response.body()).unwrap();
        assert!(response_json.data.is_some());
        assert!(response_json.errors.is_none());

        let data = response_json.data.unwrap();
        assert!(data.user_by_email.is_none());

        let body: GQLRequest<()> = GQLRequest {
            query: format!(
                r#"
                query {{
                    userById(id: "{}") {{
                        id
                        email
                        name
                        role
                        status
                    }}
                }}
            "#,
                id1
            )
            .to_string(),
            variables: None,
        };
        let response = warp::test::request()
            .method("POST")
            .json(&body)
            .filter(&filter)
            .await
            .unwrap();

        let response_json: GQLUserByIdRes = serde_json::from_slice(response.body()).unwrap();
        assert!(response_json.data.is_some());
        assert!(response_json.errors.is_none());

        let data = response_json.data.unwrap();
        assert!(data.user_by_id.is_some());

        let user = data.user_by_id.unwrap();
        assert_eq!(user.email, "test@test.com");
        assert_eq!(user.name, "test user");
        assert_eq!(user.role, "GUEST");
        assert_eq!(user.status, "ONLINE");

        let body: GQLRequest<()> = GQLRequest {
            query: r#"
                query {
                    userById(id: "12345678-1234-4123-1234-123456789012") {
                        id
                        email
                        name
                        role
                        status
                    }
                }
            "#
            .to_string(),
            variables: None,
        };
        let response = warp::test::request()
            .method("POST")
            .json(&body)
            .filter(&filter)
            .await
            .unwrap();

        let response_json: GQLUserByIdRes = serde_json::from_slice(response.body()).unwrap();
        assert!(response_json.data.is_some());
        assert!(response_json.errors.is_none());

        let data = response_json.data.unwrap();
        assert!(data.user_by_id.is_none());

        let body: GQLRequest<()> = GQLRequest {
            query: r#"
                query {
                    userById(id: "not-a-uuid") {
                        id
                        email
                        name
                        role
                        status
                    }
                }
            "#
            .to_string(),
            variables: None,
        };
        let response = warp::test::request()
            .method("POST")
            .json(&body)
            .filter(&filter)
            .await
            .unwrap();

        let response_json: GQLUserByIdRes = serde_json::from_slice(response.body()).unwrap();
        assert!(response_json.data.is_some());
        assert!(response_json.errors.is_some());

        let data = response_json.data.unwrap();
        assert!(data.user_by_id.is_none());

        let errors = response_json.errors.unwrap();
        assert_eq!(errors[0].message, "invalid character: expected an optional prefix of `urn:uuid:` followed by [0-9a-zA-Z], found `n` at 1");
        assert_eq!(errors[0].path[0], "userById");

        delete_all_users().await.unwrap();

        let body: GQLRequest<()> = GQLRequest {
            query: r#"
                query {
                    users {
                        id
                        email
                        name
                        role
                        status
                    }
            }"#
            .to_string(),
            variables: None,
        };
        let response = warp::test::request()
            .method("POST")
            .json(&body)
            .filter(&filter)
            .await
            .unwrap();

        let response_json: GQLUsersRes = serde_json::from_slice(response.body()).unwrap();
        assert!(response_json.data.is_some());
        assert!(response_json.errors.is_none());

        let data = response_json.data.unwrap();
        assert_eq!(data.users.len(), 0);
    }
}
