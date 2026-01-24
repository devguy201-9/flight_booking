use super::m20260111_201312_create_airports::Airports;
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
                    .table(Flights::Table)
                    .if_not_exists()
                    .col(pk_auto(Flights::Id))
                    .col(ColumnDef::new(Flights::AirlineCode).string().not_null())
                    .col(ColumnDef::new(Flights::FlightNumber).string().not_null())
                    .col(ColumnDef::new(Flights::FlightKey).string().not_null())
                    .col(
                        ColumnDef::new(Flights::OriginAirportId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Flights::DestinationAirportId)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Flights::DepartureDate).date().not_null())
                    .col(
                        ColumnDef::new(Flights::DepartureTime)
                            .date_time()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Flights::ArrivalTime).date_time().not_null())
                    .col(ColumnDef::new(Flights::Status).string().not_null())
                    .col(string_null(Flights::AircraftType))
                    .col(string_null(Flights::TailNumber))
                    .col(string_null(Flights::TerminalDeparture))
                    .col(string_null(Flights::TerminalArrival))
                    .col(timestamp_null(Flights::CheckinOpenAt))
                    .col(timestamp_null(Flights::CheckinCloseAt))
                    .col(timestamp_null(Flights::BoardingTime))
                    .col(string_null(Flights::Gate))
                    .col(ColumnDef::new(Flights::Gate).string().null())
                    .col(ColumnDef::new(Flights::TotalSeats).integer().not_null())
                    .col(ColumnDef::new(Flights::AvailableSeats).integer().not_null())
                    .col(
                        ColumnDef::new(Flights::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Flights::UpdatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(big_integer_null(Flights::CreatedBy))
                    .col(big_integer_null(Flights::UpdatedBy))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_flights_origin_airport")
                            .from(Flights::Table, Flights::OriginAirportId)
                            .to(Airports::Table, Airports::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_flights_destination_airport")
                            .from(Flights::Table, Flights::DestinationAirportId)
                            .to(Airports::Table, Airports::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .index(
                        Index::create()
                            .name("uq_flights_flight_key")
                            .table(Flights::Table)
                            .col(Flights::FlightKey)
                            .unique(),
                    )
                    .index(
                        Index::create()
                            .name("idx_flights_origin")
                            .table(Flights::Table)
                            .col(Flights::OriginAirportId),
                    )
                    .index(
                        Index::create()
                            .name("idx_flights_destination")
                            .table(Flights::Table)
                            .col(Flights::DestinationAirportId),
                    )
                    .index(
                        Index::create()
                            .name("idx_flights_departure_date")
                            .table(Flights::Table)
                            .col(Flights::DepartureDate),
                    )
                    .to_owned(),
            )
            .await?;

        exec_unprepared(
            manager,
            r#"
            ALTER TABLE flights
            ADD CONSTRAINT ck_flights_status
            CHECK (status IN ('SCHEDULED','DELAYED','DEPARTED','ARRIVED','CANCELLED'));
            "#,
        )
        .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Flights::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Flights {
    Table,
    Id,
    AirlineCode,
    FlightNumber,
    FlightKey,
    OriginAirportId,
    DestinationAirportId,
    DepartureDate,
    DepartureTime,
    ArrivalTime,
    Status,
    AircraftType,
    TailNumber,
    TerminalDeparture,
    TerminalArrival,
    CheckinOpenAt,
    CheckinCloseAt,
    BoardingTime,
    Gate,
    TotalSeats,
    AvailableSeats,
    CreatedAt,
    UpdatedAt,
    CreatedBy,
    UpdatedBy,
}
