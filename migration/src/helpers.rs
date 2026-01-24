use sea_orm_migration::prelude::*;

pub async fn exec_unprepared(
    manager: &SchemaManager<'_>,
    sql: &str,
) -> Result<(), DbErr> {
    manager
        .get_connection()
        .execute_unprepared(sql)
        .await?;
    Ok(())
}
