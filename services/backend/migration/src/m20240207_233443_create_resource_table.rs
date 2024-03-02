//! Represents a resource stored by the server, this could be images, files etc

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
                    .table(Resources::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Resources::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Resources::MimeType).string().not_null())
                    .col(ColumnDef::new(Resources::Name).string().not_null())
                    .col(ColumnDef::new(Resources::Description).string().null())
                    .col(ColumnDef::new(Resources::Path).string().not_null())
                    .col(ColumnDef::new(Resources::Owner).integer().not_null())
                    .col(ColumnDef::new(Resources::Visibility).integer().not_null())
                    .col(ColumnDef::new(Resources::CreatedAt).date_time().not_null())
                    // Cascade deletions from the users table onto this table
                    .foreign_key(
                        ForeignKey::create()
                            .from(Resources::Table, Resources::Owner)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Resources::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Resources {
    Table,
    /// Unique ID for the resource
    Id,
    /// The type of content
    MimeType,
    /// The content name
    Name,
    /// Optional description for the content
    Description,
    /// URL path to the resource
    Path,
    /// The user that the resource belongs to
    Owner,
    /// Public, Private
    Visibility,
    /// When the resource was created
    CreatedAt,
}
