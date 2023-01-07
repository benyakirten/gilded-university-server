#[cfg(test)]
mod integration_warp_user_mutation {
    use dotenvy::dotenv;
    use entity::sea_orm_active_enums::{Role, Status};
    use gilded_university_server::{testutils::create_test_jwt, time::Time};
    use sea_orm::prelude::Uuid;

    use crate::{
        common::{delete_all_users, get_all_users, make_graphql_filter},
        warp::{
            user::{GQLSigninRes, GQLSignoutRes, GQLSignupRes},
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
                        user {
                            id
                            email
                            name
                            status
                            role
                        }
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
                        user {
                            id
                            email
                            name
                            role
                            status
                        }
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

        let users = get_all_users().await.unwrap();
        let user1 = &users[0];

        assert_eq!(user1.email, "test@test.com");
        assert_eq!(user1.name, "test user");
        assert_eq!(user1.role, Role::Guest);
        assert_eq!(user1.status, Status::Online);

        let body: GQLRequest<()> = GQLRequest {
            query: r#"
                mutation {
                    signout(email: "test@test.com") {
                        success
                        user {
                            id
                            email
                            name
                            role
                            status
                        }
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
        assert!(response_json.errors.is_some());
        assert!(response_json.data.is_none());

        let errors = response_json.errors.unwrap();
        assert_eq!(errors[0].message, "Unable to complete request");
        assert_eq!(errors[0].path[0], "signout");

        let body: GQLRequest<()> = GQLRequest {
            query: r#"
                mutation {
                    signout(email: "test@test.com") {
                        success  
                    }
                }
            "#
            .to_string(),
            variables: None,
        };
        let token = create_test_jwt(
            &Uuid::new_v4(),
            &Role::Guest,
            Time::hour_hence().unwrap().as_secs(),
        );

        let response = warp::test::request()
            .method("POST")
            .header("Authorization", format!("Bearer {}", token))
            .json(&body)
            .filter(&filter)
            .await
            .unwrap();

        let response_json: GQLSignoutRes = serde_json::from_slice(response.body()).unwrap();
        assert!(response_json.errors.is_none());
        assert!(response_json.data.is_some());

        let errors = response_json.errors.unwrap();
        assert_eq!(errors[0].message, "Unable to complete request");
        assert_eq!(errors[0].path[0], "signout");

        let body: GQLRequest<()> = GQLRequest {
            query: r#"
                mutation {
                    signout(email: "test@test.com") {
                        success
                    }
                }
            "#
            .to_string(),
            variables: None,
        };
        let token = create_test_jwt(
            &user1.id,
            &Role::Guest,
            Time::hour_hence().unwrap().as_secs(),
        );
        let response = warp::test::request()
            .method("POST")
            .header("Authorization", format!("Bearer {}", token))
            .json(&body)
            .filter(&filter)
            .await
            .unwrap();

        let response_json: GQLSignoutRes = serde_json::from_slice(response.body()).unwrap();
        assert!(response_json.data.is_some());
        assert!(response_json.errors.is_none());

        let data = response_json.data.unwrap();
        assert!(data.signout.success);

        let users = get_all_users().await.unwrap();
        let user1 = &users[0];

        assert_eq!(user1.email, "test@test.com");
        assert_eq!(user1.name, "test user");
        assert_eq!(user1.role, Role::Guest);
        assert_eq!(user1.status, Status::Offline);

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
        assert!(response_json.data.is_none());
        assert!(response_json.errors.is_some());

        let errors = response_json.errors.unwrap();
        assert_eq!(errors[0].message, "Incorrect email or password");
        assert_eq!(errors[0].path[0], "signin");

        let body: GQLRequest<()> = GQLRequest {
            query: r#"
                mutation {
                    signin(email: "test@test.com", password: "nottherightpassword") {
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
        assert!(response_json.data.is_none());
        assert!(response_json.errors.is_some());

        let errors = response_json.errors.unwrap();
        assert_eq!(errors[0].message, "Incorrect email or password");
        assert_eq!(errors[0].path[0], "signin");

        let body: GQLRequest<()> = GQLRequest {
            query: r#"
                mutation {
                    signin(email: "test@test.com", password: "testpassword") {
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

        let users = get_all_users().await.unwrap();
        let user1 = &users[0];

        assert_eq!(user1.email, "test@test.com");
        assert_eq!(user1.name, "test user");
        assert_eq!(user1.role, Role::Guest);
        assert_eq!(user1.status, Status::Online);

        delete_all_users().await.unwrap();
    }
}
