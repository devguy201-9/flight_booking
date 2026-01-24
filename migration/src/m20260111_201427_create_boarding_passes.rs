use crate::m20260111_201358_create_checkins::Checkins;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(BoardingPasses::Table)
                    .if_not_exists()
                    .col(pk_auto(BoardingPasses::Id))
                    .col(
                        ColumnDef::new(BoardingPasses::CheckinId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(BoardingPasses::BoardingPassCode)
                            .string()
                            .not_null(),
                    )
                    .col(string_null(BoardingPasses::Gate))
                    .col(string_null(BoardingPasses::Terminal))
                    .col(string_null(BoardingPasses::BoardingGroup))
                    .col(string_null(BoardingPasses::SequenceNo))
                    .col(timestamp_null(BoardingPasses::BoardingTime))
                    .col(
                        ColumnDef::new(BoardingPasses::IssuedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(BoardingPasses::BarcodeFormat)
                            .string()
                            .not_null()
                            .default("QR"),
                    )
                    .col(text_null(BoardingPasses::BarcodePayload))
                    .col(
                        ColumnDef::new(BoardingPasses::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(BoardingPasses::UpdatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(big_integer_null(BoardingPasses::CreatedBy))
                    .col(big_integer_null(BoardingPasses::UpdatedBy))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_boarding_passes_checkin")
                            .from(BoardingPasses::Table, BoardingPasses::CheckinId)
                            .to(Checkins::Table, Checkins::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .name("uq_boarding_passes_checkin_id")
                            .table(BoardingPasses::Table)
                            .col(BoardingPasses::CheckinId)
                            .unique(),
                    )
                    .index(
                        Index::create()
                            .name("uq_boarding_passes_code")
                            .table(BoardingPasses::Table)
                            .col(BoardingPasses::BoardingPassCode)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(BoardingPasses::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum BoardingPasses {
    Table,
    Id,
    CheckinId,
    BoardingPassCode,
    Gate,
    Terminal,
    BoardingGroup,
    SequenceNo,
    BoardingTime,
    IssuedAt,
    BarcodeFormat,
    BarcodePayload,
    CreatedAt,
    UpdatedAt,
    CreatedBy,
    UpdatedBy,
}
