use super::m20260111_201209_create_users::Users;
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
                    .table(Addresses::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Addresses::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(integer(Addresses::UserId))
                    .col(string_null(Addresses::Title))
                    .col(string(Addresses::AddressLine1))
                    .col(string_null(Addresses::AddressLine2))
                    .col(string(Addresses::CountryCode))
                    .col(string(Addresses::City))
                    .col(string_null(Addresses::PostalCode))
                    .col(string_null(Addresses::PhoneNumber))
                    .col(ColumnDef::new(Addresses::Type).string().not_null())
                    .col(
                        ColumnDef::new(Addresses::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Addresses::UpdatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(big_integer_null(Addresses::CreatedBy))
                    .col(big_integer_null(Addresses::UpdatedBy))
                    .col(big_integer_null(Addresses::DeletedBy))
                    .col(timestamp_null(Addresses::DeletedAt))
                    .col(
                        ColumnDef::new(Addresses::IsDefault)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Addresses::IsDeleted)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(string_null(Addresses::RecipientName))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_addresses_user_id")
                            .from(Addresses::Table, Addresses::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        exec_unprepared(
            manager,
            r#"
            ALTER TABLE addresses
                ADD CONSTRAINT ck_addesses_type
                CHECK (type IN ('HOME','BILLING','CONTACT','OTHER'));
            "#,
        )
        .await?;

        // Create index on user_id for faster lookups
        manager
            .create_index(
                Index::create()
                    .name("idx_addresses_user_id")
                    .table(Addresses::Table)
                    .col(Addresses::UserId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Addresses::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Addresses {
    Table,
    Id,
    UserId,
    Title,
    AddressLine1,
    AddressLine2,
    CountryCode,
    City,
    IsDefault,
    RecipientName,
    PostalCode,
    PhoneNumber,
    Type,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
    CreatedBy,
    UpdatedBy,
    IsDeleted,
    DeletedBy,
}
