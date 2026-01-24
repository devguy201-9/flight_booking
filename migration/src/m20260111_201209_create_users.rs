use crate::helpers::exec_unprepared;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(pk_auto(Users::Id))
                    .col(string_null(Users::Avatar))
                    .col(string(Users::FirstName))
                    .col(string(Users::LastName))
                    .col(string_uniq(Users::Username))
                    .col(string_uniq(Users::Email))
                    .col(date_null(Users::BirthOfDate))
                    .col(string_null(Users::PhoneNumber))
                    .col(string(Users::Status).default("PENDING".to_string()))
                    .col(string(Users::Role).default("CUSTOMER".to_string()))
                    .col(boolean(Users::IsDeleted).default(false))
                    .col(string_null(Users::VerificationToken))
                    .col(timestamp_null(Users::VerificationTokenExpiry))
                    .col(timestamp_null(Users::EmailVerifiedAt))
                    .col(integer(Users::VerificationResendCount).default(0))
                    .col(timestamp_null(Users::LastVerificationResendAt))
                    .col(
                        ColumnDef::new(Users::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Users::UpdatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(big_integer_null(Users::CreatedBy))
                    .col(big_integer_null(Users::UpdatedBy))
                    .col(big_integer_null(Users::DeletedBy))
                    .col(timestamp_null(Users::DeletedAt))
                    .col(string_null(Users::PasswordHash))
                    .col(timestamp_null(Users::PasswordChangedAt))
                    .col(string_null(Users::DisplayName))
                    .col(string_null(Users::Gender))
                    .index(
                        Index::create()
                            .name("uq_users_username")
                            .table(Users::Table)
                            .col(Users::Username)
                            .unique(),
                    )
                    .index(
                        Index::create()
                            .name("uq_users_email")
                            .table(Users::Table)
                            .col(Users::Email)
                            .unique(),
                    )
                    .index(
                        Index::create()
                            .name("uq_users_phone_number")
                            .table(Users::Table)
                            .col(Users::PhoneNumber)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await?;

        exec_unprepared(
            manager,
            r#"
            ALTER TABLE users
            ADD CONSTRAINT ck_users_status
            CHECK (status IN ('ACTIVE','PENDING','SUSPENDED','DELETED'));
            "#,
        )
        .await?;

        exec_unprepared(
            manager,
            r#"
            ALTER TABLE users
            ADD CONSTRAINT ck_users_role
            CHECK (role IN ('CUSTOMER','ADMIN','STAFF'));
            "#,
        )
        .await?;
        Ok(())
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Users {
    Table,
    Id,
    Avatar,
    FirstName,
    LastName,
    Username,
    Email,
    BirthOfDate,
    PhoneNumber,
    Status,
    Role,
    IsDeleted,
    VerificationToken,
    VerificationTokenExpiry,
    EmailVerifiedAt,
    VerificationResendCount,
    LastVerificationResendAt,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
    CreatedBy,
    UpdatedBy,
    DeletedBy,
    PasswordHash,
    PasswordChangedAt,
    DisplayName,
    Gender,
}
