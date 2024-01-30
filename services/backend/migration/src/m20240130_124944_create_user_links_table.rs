//! Migration for storing the links between users and different
//! auth providers

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
                    .table(UserLinks::Table)
                    .if_not_exists()
                    // This table uses a composite key over the User ID and auth provider
                    .primary_key(
                        Index::create()
                            .col(UserLinks::UserId)
                            .col(UserLinks::Provider),
                    )
                    .col(ColumnDef::new(UserLinks::UserId).integer().not_null())
                    .col(ColumnDef::new(UserLinks::Provider).string().not_null())
                    .col(ColumnDef::new(UserLinks::CreatedAt).date_time().not_null())
                    // Cascade deletions from the users table onto this table
                    .foreign_key(
                        ForeignKey::create()
                            .from(UserLinks::Table, UserLinks::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserLinks::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum UserLinks {
    Table,
    /// The ID of the user this link belongs to
    UserId,
    /// The provider this link belongs to
    Provider,
    /// When the link was created
    CreatedAt,
}
