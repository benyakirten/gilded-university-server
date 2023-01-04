use serde::{Deserialize, Serialize};

pub mod user;

#[derive(Serialize, Deserialize, Debug)]
pub struct GQLRequest<T> {
    query: String,
    variables: Option<T>,
}

#[allow(dead_code)]
type GQLErrorResponse = GQLRequest<GQLError>;

#[derive(Serialize, Deserialize, Debug)]
pub struct GQLErrors {
    errors: Vec<GQLError>,
    path: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GQLError {
    message: String,
    locations: Vec<GQLErrorLocation>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GQLErrorLocation {
    line: usize,
    column: usize,
}

// {
//     "data":null,
//     "errors": [
//       {
//         "message":"User with emai `test2@test.com` already exists",
//         "locations": [
//           {
//             "line":3,
//             "column":17
//           }
//         ],
//       "path": [
//         "signup"
//       ]
//       }
//     ]
//   }
