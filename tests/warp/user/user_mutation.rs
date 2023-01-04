#[cfg(test)]
mod integration_warp_user_mutation {
    use dotenvy::dotenv;
    use entity::sea_orm_active_enums::{Role, Status};

    use crate::{
        common::make_graphql_filter,
        warp::{
            user::{delete_all_users, get_all_users, GQLSignoutRes, GQLSignupRes},
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
                signup(email: "test@test.com", name:"test user", password:"abc123") {
                    token  
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

        let response_json: GQLSignupRes = serde_json::from_slice(response.body()).unwrap();
        assert!(response_json.data.is_some());
        assert!(response_json.errors.is_none());

        let data = response_json.data.unwrap();
        assert!(!data.signup.token.is_empty());

        let body: GQLRequest<()> = GQLRequest {
            query: r#"
            mutation {
                signup(email: "test2@test.com", name:"test user2", password:"abc123") {
                    token  
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

        let response_json: GQLSignupRes = serde_json::from_slice(response.body()).unwrap();
        assert!(response_json.data.is_some());
        assert!(response_json.errors.is_none());

        let data = response_json.data.unwrap();
        assert!(!data.signup.token.is_empty());

        let body: GQLRequest<()> = GQLRequest {
            query: r#"
            mutation {
                signup(email: "test2@test.com", name:"test user2", password:"abc123") {
                    token  
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

        let response_json: GQLResponse<GQLSignupRes> =
            serde_json::from_slice(response.body()).unwrap();
        assert!(response_json.data.is_none());
        assert!(response_json.errors.is_some());

        let errors = response_json.errors.unwrap();
        assert_eq!(
            errors[0].message,
            "User with email `test2@test.com` already exists"
        );

        let users = get_all_users().await.unwrap();
        let user1 = &users[0];
        let user2 = &users[1];

        assert_eq!(user1.email, "test@test.com");
        assert_eq!(user1.name, "test user");
        assert_eq!(user1.role, Role::Guest);
        assert_eq!(user1.status, Status::Online);

        assert_eq!(user2.email, "test2@test.com");
        assert_eq!(user2.name, "test user2");
        assert_eq!(user2.role, Role::Guest);
        assert_eq!(user2.status, Status::Online);

        let body: GQLRequest<()> = GQLRequest {
            query: r#"
            mutation {
                signout(email: "test2@test.com") {
                    token  
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

        let response_json: GQLSignoutRes = serde_json::from_slice(response.body()).unwrap();
        assert!(response_json.data.is_some());
        assert!(response_json.errors.is_none());

        let data = response_json.data.unwrap();
        assert!(data.signout.success);

        // signout - success
        // signin - success
        // signin - failure - email doesn't exist
        // signin - failure - password incorrect
        // signout - failure - email doesn't exist
        delete_all_users().await.unwrap();

        // let x = String::from_utf8(response.body().to_vec()).unwrap();
        // println!("{}", x);
    }
}
