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
                    .table(Quiz::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Quiz::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Quiz::Title).string().not_null())
                    .col(ColumnDef::new(Quiz::Description).text().not_null())
                    .col(ColumnDef::new(Quiz::State).integer().not_null())
                    .col(ColumnDef::new(Quiz::Visibility).integer().not_null())
                    .col(ColumnDef::new(Quiz::CoverImage).text().not_null())
                    .col(ColumnDef::new(Quiz::Owner).integer().not_null())
                    .col(ColumnDef::new(Quiz::Data).json().not_null())
                    .col(ColumnDef::new(Quiz::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Quiz::UpdatedAt).date_time().not_null())
                    // Cascade deletions from the users table onto this table
                    .foreign_key(
                        ForeignKey::create()
                            .from(Quiz::Table, Quiz::Owner)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Quiz::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Quiz {
    Table,
    Id,
    /// Optional title
    Title,
    /// Optional description
    Description,
    /// Draft, Published
    State,
    /// Visibility state
    Visibility,
    /// Optional cover image for the quiz
    CoverImage,
    /// JSON quiz data
    Data,
    /// The owner of the quiz
    Owner,
    /// When the quiz was created
    CreatedAt,
    /// When the quiz was last updated
    UpdatedAt,
}
