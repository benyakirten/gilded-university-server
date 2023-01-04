#[cfg(test)]
mod integration_warp_user_mutation {
    use crate::{
        common::make_graphql_filter,
        warp::{
            user::{delete_all_users, GQLSignupRes},
            GQLRequest, GQLResponse,
        },
    };

    // To make sure the test steps perform exactly as needed
    // i.e. inserting/deleting records sequentially
    // We will use one function that will perform all the test
    #[tokio::test]
    async fn user_mutation() {
        let filter = make_graphql_filter().await;

        let body: GQLRequest<()> = GQLRequest {
            query: r#"
            mutation {
                signup(email: "test@test.com", name:"test name", password:"abc123") {
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
                signup(email: "test2@test.com", name:"test name2", password:"abc123") {
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
                signup(email: "test2@test.com", name:"test name2", password:"abc123") {
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
        // let x = String::from_utf8(response.body().to_vec()).unwrap();
        // println!("{}", x);

        let response_json: GQLResponse<GQLSignupRes> =
            serde_json::from_slice(response.body()).unwrap();
        assert!(response_json.data.is_none());
        assert!(response_json.errors.is_some());

        let errors = response_json.errors.unwrap();
        assert_eq!(
            errors[0].message,
            "User with email `test2@test.com` already exists"
        );

        // signout - success
        // signin - success
        // signin - failure - email doesn't exist
        // signin - failure - password incorrect
        // signout - failure - email doesn't exist
        // remove all records
        delete_all_users().await.unwrap();
    }
}
