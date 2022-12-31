use sea_orm_migration::prelude::{sea_query::extension::postgres::Type, *};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(Role::Table)
                    .values([Role::Guest, Role::Student, Role::Teacher, Role::Admin])
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(Status::Table)
                    .values([Status::Online, Status::Offline, Status::Hidden])
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .uuid()
                            .not_null()
                            .unique_key()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::Name).string().not_null())
                    .col(ColumnDef::new(User::Email).string().unique_key().not_null())
                    .col(ColumnDef::new(User::Password).string().not_null())
                    .col(
                        ColumnDef::new(User::Status)
                            .custom(Status::Table)
                            .not_null(),
                    )
                    .col(ColumnDef::new(User::Role).custom(Role::Table).not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_type(Type::drop().name(Status::Table).to_owned())
            .await?;
        manager
            .drop_type(Type::drop().name(Role::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum User {
    Table,
    Id,
    Name,
    Email,
    Password,
    Status,
    Role,
}

#[derive(Iden)]
pub enum Role {
    Table,
    #[iden = "Guest"]
    Guest,
    #[iden = "Student"]
    Student,
    #[iden = "Teacher"]
    Teacher,
    #[iden = "Admin"]
    Admin,
}

#[derive(Iden)]
pub enum Status {
    Table,
    #[iden = "Offline"]
    Offline,
    #[iden = "Online"]
    Online,
    #[iden = "Hidden"]
    Hidden,
}
