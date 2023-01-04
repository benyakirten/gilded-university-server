use sea_orm::{
    prelude::Uuid, ActiveValue, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, InsertResult,
    QueryFilter,
};

use crate::{
    prelude::User,
    sea_orm_active_enums::{Role, Status},
    user::{self, ActiveModel},
};

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

    // All following traits are tested in integration dataase tests
    pub async fn find_one_by_email(
        email: &str,
        conn: &DatabaseConnection,
    ) -> Result<Option<user::Model>, DbErr> {
        User::find()
            .filter(user::Column::Email.eq(email.to_string()))
            .one(conn)
            .await
    }

    pub async fn find_one_by_id(
        id: &Uuid,
        conn: &DatabaseConnection,
    ) -> Result<Option<user::Model>, DbErr> {
        User::find_by_id(*id).one(conn).await
    }

    pub async fn find_all(conn: &DatabaseConnection) -> Result<Vec<user::Model>, DbErr> {
        User::find().all(conn).await
    }

    // These methods are fairly simple but are their own methods
    // so that they can be more concisely tested
    pub async fn insert_one(
        model: user::ActiveModel,
        conn: &DatabaseConnection,
    ) -> Result<InsertResult<user::ActiveModel>, sea_orm::DbErr> {
        user::Entity::insert(model).exec(conn).await
    }

    pub async fn update_one(
        model: user::ActiveModel,
        conn: &DatabaseConnection,
    ) -> Result<user::Model, DbErr> {
        User::update(model).exec(conn).await
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
