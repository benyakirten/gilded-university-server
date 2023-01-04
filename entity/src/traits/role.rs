use std::str::FromStr;

use thiserror::Error;

use crate::sea_orm_active_enums::Role;

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
