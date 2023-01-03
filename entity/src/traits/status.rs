use crate::sea_orm_active_enums::Status;

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
