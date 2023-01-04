#[cfg(test)]
mod integration_warp_user_mutation {
    use crate::{
        common::make_graphql_filter,
        warp::{
            user::{delete_all_users, GQLSignupRes},
            GQLRequest,
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
        assert!(!response_json.data.signup.token.is_empty());

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
        assert!(!response_json.data.signup.token.is_empty());

        // signup - failure - email already exists
        // signout - success
        // signin - success
        // signin - failure - email doesn't exist
        // signin - failure - password incorrect
        // signout - failure - email doesn't exist
        // remove all records
        delete_all_users().await.unwrap();
    }
}
