#[cfg(test)]
mod integration_warp_user_query {
    // To make sure the test steps perform exactly as needed
    // i.e. inserting/deleting records sequentially
    // We will use one function that will perform all the test
    #[tokio::test]
    async fn user_mutation() {
        // create database connection
        // seed database with multiple users
        // find user by email - success
        // find user by email - no result
        // find user by id - success
        // find user by id - no result
        // get users - success
        // remove user records
        // get users - no results
    }
}
