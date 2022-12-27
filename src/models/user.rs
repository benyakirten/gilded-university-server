use std::fmt::{Display, Formatter, Result};

#[derive(Clone, PartialEq, PartialOrd)]
pub enum Role {
    Guest,
    Student,
    Teacher,
    Admin,
}

impl Role {
    pub fn from_str(role: &str) -> Role {
        match role {
            "Student" => Role::Student,
            "Teacher" => Role::Teacher,
            "Admin" => Role::Admin,
            _ => Role::Guest,
        }
    }

    pub fn meets_requirements(&self, requirement: &Role) -> bool {
        self >= requirement
    }
}

impl Display for Role {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Role::Guest => write!(f, "Guest"),
            Role::Student => write!(f, "Student"),
            Role::Teacher => write!(f, "Teacher"),
            Role::Admin => write!(f, "Admin"),
        }
    }
}
