#[cfg(test)]
mod integration_warp_user_integration {
    // To make sure the test steps perform exactly as needed
    // i.e. inserting/deleting records sequentially
    // We will use one function that will perform all the test
    #[tokio::test]
    async fn user_mutation() {
        // create database connection
        // signup - success (user 1)
        // signup - failure - email already exists
        // signup - success (user 2)
        // find user by email - success (user 2)
        // find user by email - empty result (incorrect email)
        // signout - success (user 1)
        // find user by id - success (user 1 - verify they are offline)
        // signout - failure
        // signin - success (user 1)
        // get users - success (see both are online)
        // signout - success (user 2)
        // get users - success (see user 1 is online, user 2 is offline)
        // delete records
        // get users - empty results
    }
}
