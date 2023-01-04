#[cfg(test)]
mod integration_warp_user_integration {
    use dotenvy::dotenv;

    use crate::{
        common::make_graphql_filter,
        warp::{
            user::{
                delete_all_users, GQLSigninRes, GQLSignoutRes, GQLSignupRes, GQLUserByEmailRes,
                GQLUserByIdRes, GQLUsersRes,
            },
            GQLRequest, GQLResponse,
        },
    };

    // To make sure the test steps perform exactly as needed
    // i.e. inserting/deleting records sequentially
    // We will use one function that will perform all the test
    #[tokio::test]
    async fn user_mutation() {
        dotenv().ok();
        let filter = make_graphql_filter().await;

        let body: GQLRequest<()> = GQLRequest {
            query: r#"
                mutation {
                    signup(email: "test@test.com", name:"test user", password:"testpassword") {
                        token  
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

        let response_json: GQLSignupRes = serde_json::from_slice(response.body()).unwrap();
        assert!(response_json.data.is_some());
        assert!(response_json.errors.is_none());

        let data = response_json.data.unwrap();
        assert!(!data.signup.token.is_empty());

        let body: GQLRequest<()> = GQLRequest {
            query: r#"
                mutation {
                    signup(email: "test@test.com", name:"test user2", password:"testpassword") {
                        token  
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

        let response_json: GQLResponse<GQLSignupRes> =
            serde_json::from_slice(response.body()).unwrap();
        assert!(response_json.data.is_none());
        assert!(response_json.errors.is_some());

        let errors = response_json.errors.unwrap();
        assert_eq!(errors[0].message, "Unable to complete request");
        assert_eq!(errors[0].path[0], "signup");

        let body: GQLRequest<()> = GQLRequest {
            query: r#"
                mutation {
                    signup(email: "test2@test.com", name:"test user2", password:"testpassword") {
                        token  
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

        let response_json: GQLSignupRes = serde_json::from_slice(response.body()).unwrap();
        assert!(response_json.data.is_some());
        assert!(response_json.errors.is_none());

        let data = response_json.data.unwrap();
        assert!(!data.signup.token.is_empty());

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

        let id2 = data.users[1].id.clone();

        assert_eq!(user1.email, "test@test.com");
        assert_eq!(user1.name, "test user");
        assert_eq!(user1.role, "GUEST");
        assert_eq!(user1.status, "ONLINE");

        assert_eq!(user2.email, "test2@test.com");
        assert_eq!(user2.name, "test user2");
        assert_eq!(user2.role, "GUEST");
        assert_eq!(user2.status, "ONLINE");

        let body: GQLRequest<()> = GQLRequest {
            query: r#"
                mutation {
                    signout(email: "test2@test.com") {
                        success  
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

        let response_json: GQLSignoutRes = serde_json::from_slice(response.body()).unwrap();
        assert!(response_json.data.is_some());
        assert!(response_json.errors.is_none());

        let data = response_json.data.unwrap();
        assert!(data.signout.success);

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
        assert_eq!(user.role, "GUEST");
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
            query: r#"
                mutation {
                    signin(email: "test2@test.com", password: "testpassword") {
                        token  
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

        let response_json: GQLSigninRes = serde_json::from_slice(response.body()).unwrap();
        assert!(response_json.data.is_some());
        assert!(response_json.errors.is_none());

        let data = response_json.data.unwrap();
        assert!(!data.signin.token.is_empty());

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
                id2
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
        assert_eq!(user.email, "test2@test.com");
        assert_eq!(user.name, "test user2");
        assert_eq!(user.role, "GUEST");
        assert_eq!(user.status, "ONLINE");

        delete_all_users().await.unwrap();
    }
}
