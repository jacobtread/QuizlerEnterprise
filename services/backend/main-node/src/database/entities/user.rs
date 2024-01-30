use crate::database::DbResult;
use chrono::Utc;
use sea_orm::IntoActiveModel;
use sea_orm::{entity::prelude::*, ActiveValue};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ConnectionTrait};
use std::future::Future;

pub type User = Model;
pub type UserEntity = Entity;
pub type UserActiveModel = ActiveModel;

/// Database structure for a user
#[derive(Debug, Clone, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    /// Unique ID for the user
    #[sea_orm(primary_key)]
    pub id: u32,
    /// Email address for the user
    #[sea_orm(unique)]
    pub email: String,
    /// When the email address was verified, if it was verified
    pub email_verified_at: Option<DateTimeUtc>,
    /// The account username
    pub username: String,
    /// The password associated with this account
    pub password: Option<String>,
    /// The role for this user
    pub role: UserRole,
    /// When this user was created
    pub created_at: DateTimeUtc,
    /// When the last change was made to this user
    pub updated_at: DateTimeUtc,
}

#[derive(Debug, Clone, Default, EnumIter, PartialEq, DeriveActiveEnum)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum UserRole {
    #[default]
    #[sea_orm(num_value = 0)]
    Standard,
    #[sea_orm(num_value = 1)]
    Moderator,
    #[sea_orm(num_value = 2)]
    Administrator,
}

#[derive(DeriveIntoActiveModel)]
pub struct CreateUser {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::user_link::Entity")]
    UserLinks,
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    /// Handles updating the `updated_at` field before the model is saved, using
    /// the current date time.
    ///
    /// If the save is an insertion the `created_at` field will also be updated
    async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        let now = Utc::now();
        self.updated_at = ActiveValue::Set(now);

        if insert {
            self.created_at = ActiveValue::Set(now);
        }

        Ok(self)
    }
}

impl Model {
    /// Create a new user
    pub fn create<C>(db: &C, mut create: CreateUser) -> impl Future<Output = DbResult<User>> + '_
    where
        C: ConnectionTrait,
    {
        // Ensure the email is in lowercase
        create.email = create.email.to_lowercase();

        create.into_active_model().insert(db)
    }

    /// Finds a user by email if a matching email exists
    pub fn find_by_email<'db, C>(
        db: &'db C,
        email: &str,
    ) -> impl Future<Output = DbResult<Option<User>>> + 'db
    where
        C: ConnectionTrait,
    {
        // Ensure the email is in lowercase
        let email = email.to_lowercase();

        Entity::find().filter(Column::Email.eq(email)).one(db)
    }

    /// Sets the email for the provided user to verified at the
    /// current time
    pub fn set_email_verified<C>(self, db: &C) -> impl Future<Output = DbResult<User>> + '_
    where
        C: ConnectionTrait,
    {
        let mut model = self.into_active_model();
        model.email_verified_at = Set(Some(Utc::now()));
        model.update(db)
    }
}

impl Related<super::user_link::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserLinks.def()
    }
}
