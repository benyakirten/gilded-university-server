use std::str::FromStr;

use sea_orm::prelude::Uuid;
use sea_orm::ActiveValue;

use crate::sea_orm_active_enums::{Role, Status};
use crate::{prelude::User, user::ActiveModel};

impl Role {
    pub fn meets_requirements(&self, role: &Role) -> bool {
        self.to_int() >= role.to_int()
    }

    pub fn to_int(&self) -> u8 {
        match self {
            Role::Admin => 3,
            Role::Teacher => 2,
            Role::Student => 1,
            Role::Guest => 0,
        }
    }

    pub fn to_str(&self) -> String {
        match self {
            Role::Admin => "Admin".to_string(),
            Role::Teacher => "Teacher".to_string(),
            Role::Student => "Student".to_string(),
            Role::Guest => "Guest".to_string(),
        }
    }
}
#[derive(Debug)]
pub struct UnexpetedError {}

impl FromStr for Role {
    type Err = UnexpetedError;
    fn from_str(role: &str) -> Result<Self, Self::Err> {
        let role = match role {
            "Admin" => Role::Admin,
            "Teacher" => Role::Teacher,
            "Student" => Role::Student,
            _ => Role::Guest,
        };
        Ok(role)
    }
}

impl Status {
    pub fn to_str(&self) -> String {
        match self {
            Status::Online => "Online".to_string(),
            Status::Offline => "Offline".to_string(),
            Status::Hidden => "Hidden".to_string(),
        }
    }
}

impl User {
    pub fn create_active_model(email: &str, name: &str, password: &str) -> ActiveModel {
        ActiveModel {
            id: ActiveValue::Set(Uuid::new_v4()),
            email: ActiveValue::Set(email.to_string()),
            name: ActiveValue::Set(name.to_string()),
            password: ActiveValue::Set(password.to_string()),
            role: ActiveValue::Set(Role::Guest),
            status: ActiveValue::Set(Status::Online),
        }
    }
}
