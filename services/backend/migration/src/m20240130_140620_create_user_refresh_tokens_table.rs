//! Migration for table that stores "refresh" tokens

use sea_orm_migration::prelude::*;

use crate::m20240128_142246_create_users_table::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserRefreshTokens::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserRefreshTokens::RefreshToken)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(UserRefreshTokens::UserId)
                            .integer()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(UserRefreshTokens::CreatedAt)
                            .date_time()
                            .not_null(),
                    )
                    // Cascade deletions from the users table onto this table
                    .foreign_key(
                        ForeignKey::create()
                            .from(UserRefreshTokens::Table, UserRefreshTokens::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserRefreshTokens::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum UserRefreshTokens {
    Table,
    /// The ID of the user the refresh token belongs to
    UserId,
    /// The refresh token itself
    RefreshToken,
    /// When the token was created
    CreatedAt,
}
