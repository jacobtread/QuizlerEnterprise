use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::ServiceId).string().not_null())
                    .col(ColumnDef::new(User::Email).string().not_null())
                    .col(ColumnDef::new(User::EmailVerifiedAt).date_time().null())
                    .col(
                        ColumnDef::new(User::Username)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(User::Password).string().not_null())
                    .col(ColumnDef::new(User::Role).integer().not_null())
                    .col(ColumnDef::new(User::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(User::UpdatedAt).date_time().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum User {
    Table,
    /// Unique ID for the user
    Id,
    /// String encoded ID with additional information about the service
    /// that created the user if one was used
    ServiceId,
    /// Email address of the user
    Email,
    /// When the email was verified
    EmailVerifiedAt,
    /// The username of the user
    Username,
    /// The user password hash
    Password,
    /// User role, for additional permissions
    Role,
    /// When the user was created
    CreatedAt,
    /// When the user was last updated
    UpdatedAt,
}
