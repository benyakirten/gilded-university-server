use std::str::FromStr;

use crate::sea_orm_active_enums::Role;

impl Role {
    pub fn meets_requirements(&self, role: &Role) -> bool {
        self.to_int() >= role.to_int()
    }

    fn to_int(&self) -> u8 {
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
