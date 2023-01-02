use std::str::FromStr;

use sea_orm::prelude::Uuid;
use sea_orm::ActiveValue;
use thiserror::Error;

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
#[derive(Error, Debug)]
pub enum ParsingError {
    #[error("No role corresponding to {0}")]
    NoRoleForStr(String),
}

impl FromStr for Role {
    type Err = ParsingError;
    fn from_str(role: &str) -> Result<Self, Self::Err> {
        match role {
            "Admin" => Ok(Role::Admin),
            "Teacher" => Ok(Role::Teacher),
            "Student" => Ok(Role::Student),
            "Guest" => Ok(Role::Guest),
            _ => Err(ParsingError::NoRoleForStr(role.to_string())),
        }
    }
}

#[cfg(test)]
mod test_role {
    use std::str::FromStr;

    use crate::sea_orm_active_enums::Role;

    #[test]
    fn expect_results_from_to_int() {
        assert_eq!(Role::Admin.to_int(), 3);
        assert_eq!(Role::Teacher.to_int(), 2);
        assert_eq!(Role::Student.to_int(), 1);
        assert_eq!(Role::Guest.to_int(), 0);
    }

    #[test]
    fn expect_results_from_to_str() {
        assert_eq!(Role::Admin.to_str(), "Admin");
        assert_eq!(Role::Teacher.to_str(), "Teacher");
        assert_eq!(Role::Student.to_str(), "Student");
        assert_eq!(Role::Guest.to_str(), "Guest");
    }

    #[test]
    fn expect_results_from_from_str() {
        assert_eq!(Role::from_str("Admin").unwrap(), Role::Admin);
        assert_eq!(Role::from_str("Teacher").unwrap(), Role::Teacher);
        assert_eq!(Role::from_str("Student").unwrap(), Role::Student);
        assert_eq!(Role::from_str("Guest").unwrap(), Role::Guest);
    }

    #[test]
    fn expect_err_from_from_str() {
        let got = Role::from_str("notarole");
        assert!(got.is_err());

        let err = got.err().unwrap().to_string();
        assert_eq!(err, "No role corresponding to notarole");
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

#[cfg(test)]
mod test_status {
    use crate::sea_orm_active_enums::Status;

    #[test]
    fn expect_results_from_to_str() {
        assert_eq!(Status::Online.to_str(), "Online");
        assert_eq!(Status::Offline.to_str(), "Offline");
        assert_eq!(Status::Hidden.to_str(), "Hidden");
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

#[cfg(test)]
mod test_user {
    use crate::{
        prelude::User,
        sea_orm_active_enums::{Role, Status},
    };

    #[test]
    fn create_model_from_data() {
        let got = User::create_active_model("test@test.com", "test user", "testpassword");

        assert_eq!(got.email.unwrap(), "test@test.com");
        assert_eq!(got.name.unwrap(), "test user");
        assert_eq!(got.password.unwrap(), "testpassword");
        assert_eq!(got.role.unwrap(), Role::Guest);
        assert_eq!(got.status.unwrap(), Status::Online);

        let id = got.id.unwrap();
        assert!(!id.is_nil());
    }
}
