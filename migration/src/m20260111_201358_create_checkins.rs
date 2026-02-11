use crate::helpers::exec_unprepared;
use crate::m20260111_201326_create_booking::Bookings;
use crate::m20260111_201350_create_passengers::Passengers;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Checkins::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Checkins::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Checkins::BookingId).integer().not_null())
                    .col(ColumnDef::new(Checkins::PassengerId).integer().not_null())
                    .col(string_null(Checkins::SeatNo))
                    .col(ColumnDef::new(Checkins::SeatClass).string().not_null())
                    .col(ColumnDef::new(Checkins::Status).string().not_null())
                    .col(
                        ColumnDef::new(Checkins::BaggageCount)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Checkins::BaggageWeightTotal)
                            .decimal()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Checkins::BaggageWeightUnit)
                            .string()
                            .not_null()
                            .default("KG"),
                    )
                    .col(timestamp_null(Checkins::CheckedInAt))
                    .col(ColumnDef::new(Checkins::CheckinChannel).string().not_null())
                    .col(string_null(Checkins::CheckedInIp))
                    .col(ColumnDef::new(Checkins::Version).integer().default(1))
                    .col(
                        ColumnDef::new(Checkins::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Checkins::UpdatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(big_integer_null(Checkins::CreatedBy))
                    .col(big_integer_null(Checkins::UpdatedBy))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_checkins_booking")
                            .from(Checkins::Table, Checkins::BookingId)
                            .to(Bookings::Table, Bookings::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_checkins_passenger")
                            .from(Checkins::Table, Checkins::PassengerId)
                            .to(Passengers::Table, Passengers::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .name("uq_checkins_booking_passenger")
                            .table(Checkins::Table)
                            .col(Checkins::BookingId)
                            .col(Checkins::PassengerId)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_checkins_booking_id")
                    .table(Checkins::Table)
                    .col(Checkins::BookingId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_checkins_passenger_id")
                    .table(Checkins::Table)
                    .col(Checkins::PassengerId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_version")
                    .table(Checkins::Table)
                    .col(Checkins::Version)
                    .to_owned(),
            )
            .await?;

        exec_unprepared(
            manager,
            r#"
                ALTER TABLE checkins
                ADD CONSTRAINT ck_checkins_status
                CHECK (status IN ('PENDING','CHECKED_IN','CANCELLED'));
                "#,
        )
        .await?;
        exec_unprepared(
            manager,
            r#"
                ALTER TABLE checkins
                ADD CONSTRAINT ck_checkins_seat_class
                CHECK (seat_class IN ('ECONOMY','PREMIUM_ECONOMY','BUSINESS','FIRST'));
                "#,
        )
        .await?;
        exec_unprepared(
            manager,
            r#"
                ALTER TABLE checkins
                ADD CONSTRAINT ck_checkins_channel
                CHECK (checkin_channel IN ('WEB','MOBILE','COUNTER','KIOSK'));
                "#,
        )
        .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Checkins::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Checkins {
    Table,
    Id,
    BookingId,
    PassengerId,
    SeatNo,
    SeatClass,
    Status,
    BaggageCount,
    BaggageWeightTotal,
    BaggageWeightUnit,
    CheckedInAt,
    CheckinChannel,
    CheckedInIp,
    Version,
    CreatedAt,
    UpdatedAt,
    CreatedBy,
    UpdatedBy,
}
