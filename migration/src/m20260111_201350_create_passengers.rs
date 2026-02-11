use crate::helpers::exec_unprepared;
use crate::m20260111_201326_create_booking::Bookings;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Passengers::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Passengers::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Passengers::BookingId).integer().not_null())
                    .col(
                        ColumnDef::new(Passengers::PassengerType)
                            .string()
                            .not_null(),
                    )
                    .col(string_null(Passengers::Title))
                    .col(ColumnDef::new(Passengers::FirstName).string().not_null())
                    .col(ColumnDef::new(Passengers::LastName).string().not_null())
                    .col(ColumnDef::new(Passengers::Dob).date().not_null())
                    .col(ColumnDef::new(Passengers::Gender).string().not_null())
                    .col(
                        ColumnDef::new(Passengers::NationalityCode)
                            .string()
                            .not_null(),
                    )
                    .col(string_null(Passengers::PassportNo))
                    .col(date_null(Passengers::PassportExpiryDate))
                    .col(string_null(Passengers::PassportIssuingCountryCode))
                    .col(string_null(Passengers::Email))
                    .col(string_null(Passengers::PhoneNumber))
                    .col(string_null(Passengers::FfAirlineCode))
                    .col(string_null(Passengers::FfNumber))
                    .col(ColumnDef::new(Passengers::Version).integer().default(1))
                    .col(
                        ColumnDef::new(Passengers::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Passengers::UpdatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(big_integer_null(Passengers::CreatedBy))
                    .col(big_integer_null(Passengers::UpdatedBy))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_passengers_booking")
                            .from(Passengers::Table, Passengers::BookingId)
                            .to(Bookings::Table, Bookings::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_passengers_booking_id")
                    .table(Passengers::Table)
                    .col(Passengers::BookingId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_version")
                    .table(Passengers::Table)
                    .col(Passengers::Version)
                    .to_owned(),
            )
            .await?;

        exec_unprepared(
            manager,
            r#"
                ALTER TABLE passengers
                ADD CONSTRAINT ck_passengers_type
                CHECK (passenger_type IN ('ADT','CHD','INF'));
                "#,
        )
        .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Passengers::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Passengers {
    Table,
    Id,
    BookingId,
    PassengerType,
    Title,
    FirstName,
    LastName,
    Dob,
    Gender,
    NationalityCode,
    PassportNo,
    PassportExpiryDate,
    PassportIssuingCountryCode,
    Email,
    PhoneNumber,
    FfAirlineCode,
    FfNumber,
    Version,
    CreatedAt,
    UpdatedAt,
    CreatedBy,
    UpdatedBy,
}
