use crate::helpers::exec_unprepared;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // failed_login_attempts
        exec_unprepared(
            manager,
            r#"
            ALTER TABLE users
            ADD COLUMN IF NOT EXISTS failed_login_attempts integer NOT NULL DEFAULT 0;
            "#,
        )
        .await?;

        // last_failed_login_at
        exec_unprepared(
            manager,
            r#"
            ALTER TABLE users
            ADD COLUMN IF NOT EXISTS last_failed_login_at timestamp NULL;
            "#,
        )
        .await?;

        // account_locked_until
        exec_unprepared(
            manager,
            r#"
            ALTER TABLE users
            ADD COLUMN IF NOT EXISTS account_locked_until timestamp NULL;
            "#,
        )
        .await?;

        // last_login_at
        exec_unprepared(
            manager,
            r#"
            ALTER TABLE users
            ADD COLUMN IF NOT EXISTS last_login_at timestamp NULL;
            "#,
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        exec_unprepared(
            manager,
            r#"
            ALTER TABLE users
            DROP COLUMN IF EXISTS last_login_at;
            "#,
        )
        .await?;

        exec_unprepared(
            manager,
            r#"
            ALTER TABLE users
            DROP COLUMN IF EXISTS account_locked_until;
            "#,
        )
        .await?;

        exec_unprepared(
            manager,
            r#"
            ALTER TABLE users
            DROP COLUMN IF EXISTS last_failed_login_at;
            "#,
        )
        .await?;

        exec_unprepared(
            manager,
            r#"
            ALTER TABLE users
            DROP COLUMN IF EXISTS failed_login_attempts;
            "#,
        )
        .await?;

        Ok(())
    }
}
