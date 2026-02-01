use crate::helpers::exec_unprepared;
use crate::m20260111_201209_create_users::Users;
use crate::m20260111_201319_create_flights::Flights;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Bookings::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Bookings::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Bookings::BookingCode).string().not_null())
                    .col(ColumnDef::new(Bookings::UserId).integer().not_null())
                    .col(ColumnDef::new(Bookings::FlightId).integer().not_null())
                    .col(ColumnDef::new(Bookings::Status).string().not_null())
                    .col(string_null(Bookings::CancellationReason))
                    .col(ColumnDef::new(Bookings::BaseAmount).decimal().not_null())
                    .col(ColumnDef::new(Bookings::TaxesAmount).decimal().not_null())
                    .col(ColumnDef::new(Bookings::FeesAmount).decimal().not_null())
                    .col(
                        ColumnDef::new(Bookings::DiscountAmount)
                            .decimal()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new(Bookings::TotalAmount).decimal().not_null())
                    .col(ColumnDef::new(Bookings::Currency).string().not_null())
                    .col(ColumnDef::new(Bookings::ContactEmail).string().not_null())
                    .col(string_null(Bookings::ContactPhone))
                    .col(
                        ColumnDef::new(Bookings::ContactFullName)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Bookings::PaymentStatus).string().not_null())
                    .col(string_null(Bookings::PaymentMethod))
                    .col(string_null(Bookings::PaymentTxnId))
                    .col(timestamp_null(Bookings::PaidAt))
                    .col(
                        ColumnDef::new(Bookings::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Bookings::UpdatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(big_integer_null(Bookings::CreatedBy))
                    .col(big_integer_null(Bookings::UpdatedBy))
                    .col(big_integer_null(Bookings::CancelledBy))
                    .col(timestamp_null(Bookings::ConfirmedAt))
                    .col(timestamp_null(Bookings::CancelledAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_bookings_user")
                            .from(Bookings::Table, Bookings::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_bookings_flight")
                            .from(Bookings::Table, Bookings::FlightId)
                            .to(Flights::Table, Flights::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .index(
                        Index::create()
                            .name("uq_bookings_booking_code")
                            .table(Bookings::Table)
                            .col(Bookings::BookingCode)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_bookings_user_id")
                    .table(Bookings::Table)
                    .col(Bookings::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_bookings_flight_id")
                    .table(Bookings::Table)
                    .col(Bookings::FlightId)
                    .to_owned(),
            )
            .await?;

        exec_unprepared(
            manager,
            r#"
            ALTER TABLE bookings
                ADD CONSTRAINT ck_bookings_status
                CHECK (status IN ('DRAFT','CONFIRMED','CANCELLED','EXPIRED'));
            "#,
        )
        .await?;

        exec_unprepared(
            manager,
            r#"
            ALTER TABLE bookings
                ADD CONSTRAINT ck_bookings_payment_status
                CHECK (payment_status IN ('UNPAID','PAID','REFUNDED','PARTIAL_REFUND'));
            "#,
        )
        .await?;

        exec_unprepared(
            manager,
            r#"
            ALTER TABLE bookings
                ADD CONSTRAINT ck_bookings_payment_method
                CHECK (payment_method IS NULL OR payment_method IN ('CARD','BANK_TRANSFER','WALLET'));
            "#,
        )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Bookings::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Bookings {
    Table,
    Id,
    BookingCode,
    UserId,
    FlightId,
    Status,
    CancellationReason,
    BaseAmount,
    TaxesAmount,
    FeesAmount,
    DiscountAmount,
    TotalAmount,
    Currency,
    ContactEmail,
    ContactPhone,
    ContactFullName,
    PaymentStatus,
    PaymentMethod,
    PaymentTxnId,
    PaidAt,
    CreatedAt,
    UpdatedAt,
    CreatedBy,
    UpdatedBy,
    ConfirmedAt,
    CancelledAt,
    CancelledBy,
}
