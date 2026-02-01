use crate::helpers::exec_unprepared;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add verification_resend_count (idempotent)
        exec_unprepared(
            manager,
            r#"
            ALTER TABLE users
            ADD COLUMN IF NOT EXISTS verification_resend_count integer NOT NULL DEFAULT 0;
            "#,
        )
        .await?;

        // Add last_verification_resend_at (idempotent)
        exec_unprepared(
            manager,
            r#"
            ALTER TABLE users
            ADD COLUMN IF NOT EXISTS last_verification_resend_at timestamp NULL;
            "#,
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop last_verification_resend_at (safe)
        exec_unprepared(
            manager,
            r#"
            ALTER TABLE users
            DROP COLUMN IF EXISTS last_verification_resend_at;
            "#,
        )
        .await?;

        // Drop verification_resend_count (safe)
        exec_unprepared(
            manager,
            r#"
            ALTER TABLE users
            DROP COLUMN IF EXISTS verification_resend_count;
            "#,
        )
        .await?;

        Ok(())
    }
}
