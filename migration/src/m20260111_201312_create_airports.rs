use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Airports::Table)
                    .if_not_exists()
                    .col(pk_auto(Airports::Id))
                    .col(ColumnDef::new(Airports::IataCode).string().not_null())
                    .col(string_null(Airports::IcaoCode))
                    .col(ColumnDef::new(Airports::Name).string().not_null())
                    .col(ColumnDef::new(Airports::City).string().not_null())
                    .col(ColumnDef::new(Airports::CountryCode).string().not_null())
                    .col(ColumnDef::new(Airports::Timezone).string().not_null())
                    .col(ColumnDef::new(Airports::Latitude).decimal().null())
                    .col(ColumnDef::new(Airports::Longitude).decimal().null())
                    .col(
                        ColumnDef::new(Airports::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .index(
                        Index::create()
                            .name("uq_airports_iata_code")
                            .table(Airports::Table)
                            .col(Airports::IataCode)
                            .unique(),
                    )
                    .index(
                        Index::create()
                            .name("uq_airports_icao_code")
                            .table(Airports::Table)
                            .col(Airports::IcaoCode)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Airports::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Airports {
    Table,
    Id,
    IataCode,
    IcaoCode,
    Name,
    City,
    CountryCode,
    Timezone,
    Latitude,
    Longitude,
    IsActive,
}
