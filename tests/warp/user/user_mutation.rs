#[cfg(test)]

mod integration_warp_user_mutation {
    use crate::common::make_graphql_filter;

    // To make sure the test steps perform exactly as needed
    // i.e. inserting/deleting records sequentially
    // We will use one function that will perform all the test
    #[tokio::test]
    async fn user_mutation() {
        let filter = make_graphql_filter().await;

        let response = warp::test::request()
            .method("POST")
            // .body(body)
            .filter(&filter)
            .await
            .unwrap();
        // assert!(false);
        // signup - success
        // signup - failure - email already exists
        // signout - success
        // signin - success
        // signin - failure - email doesn't exist
        // signin - failure - password incorrect
        // signout - failure - email doesn't exist
        // remove all records
    }
}
