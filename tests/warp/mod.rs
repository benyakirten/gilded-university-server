use serde::{Deserialize, Serialize};

pub mod user;

#[derive(Serialize, Deserialize, Debug)]
pub struct GQLRequest<T> {
    query: String,
    variables: Option<T>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GQLResponse<T: Serialize> {
    pub data: Option<T>,
    pub errors: Option<Vec<GQLError>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GQLError {
    pub message: String,
    pub locations: Vec<GQLErrorLocation>,
    pub path: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GQLErrorLocation {
    pub line: usize,
    pub column: usize,
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
