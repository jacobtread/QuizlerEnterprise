use crate::database::DbResult;
use crate::utils::types::EmailAddress;
use chrono::Utc;
use sea_orm::{entity::prelude::*, ActiveValue};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ConnectionTrait};
use sea_orm::{IntoActiveModel, QuerySelect, SelectColumns};
use serde::{Deserialize, Serialize};
use std::future::Future;

pub type User = Model;
pub type UserEntity = Entity;
pub type UserActiveModel = ActiveModel;

pub type UserId = i32;

/// Database structure for a user
#[derive(Debug, Clone, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    /// Unique ID for the user
    #[sea_orm(primary_key)]
    #[serde(skip)]
    pub id: UserId,
    /// Email address for the user
    #[sea_orm(unique)]
    pub email: String,
    /// When the email address was verified, if it was verified
    pub email_verified_at: Option<DateTime>,
    /// The account username
    pub username: String,
    /// The password associated with this account
    #[serde(skip)]
    pub password: Option<String>,
    /// The role for this user
    pub role: UserRole,
    /// When this user was created
    pub created_at: DateTime,
    /// When the last change was made to this user
    pub updated_at: DateTime,
}

#[derive(Debug, Clone, Default, EnumIter, PartialEq, DeriveActiveEnum, Serialize, Deserialize)]
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
    #[sea_orm(has_one = "super::user_refresh_token::Entity")]
    RefreshToken,
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
        let now = Utc::now().naive_utc();
        self.updated_at = ActiveValue::Set(now);

        if insert {
            self.created_at = ActiveValue::Set(now);
        }

        Ok(self)
    }
}

impl Model {
    /// Create a new user
    pub fn create<C>(db: &C, create: CreateUser) -> impl Future<Output = DbResult<User>> + '_
    where
        C: ConnectionTrait,
    {
        create.into_active_model().insert(db)
    }

    /// Finds a user by its ID
    pub fn find_by_id<C>(db: &C, id: UserId) -> impl Future<Output = DbResult<Option<User>>> + '_
    where
        C: ConnectionTrait,
    {
        Entity::find_by_id(id).one(db)
    }

    /// Finds a user by email if a matching email exists
    pub fn find_by_email<'db, C>(
        db: &'db C,
        email: &EmailAddress,
    ) -> impl Future<Output = DbResult<Option<User>>> + 'db
    where
        C: ConnectionTrait,
    {
        Entity::find()
            .filter(Column::Email.eq(email.as_str()))
            .one(db)
    }

    pub async fn is_email_taken<C>(db: &C, email: &EmailAddress) -> DbResult<bool>
    where
        C: ConnectionTrait,
    {
        Entity::find()
            .filter(Column::Email.eq(email.as_str()))
            .select_only()
            .select_column(Column::Email)
            .count(db)
            .await
            .map(|value| value > 0)
    }

    pub async fn is_username_taken<C>(db: &C, username: &str) -> DbResult<bool>
    where
        C: ConnectionTrait,
    {
        Entity::find()
            .filter(Column::Username.eq(username))
            .select_only()
            .select_column(Column::Username)
            .count(db)
            .await
            .map(|value| value > 0)
    }

    /// Sets the email for the provided user to verified at the
    /// current time
    pub fn set_email_verified<C>(self, db: &C) -> impl Future<Output = DbResult<User>> + '_
    where
        C: ConnectionTrait,
    {
        let mut model = self.into_active_model();
        model.email_verified_at = Set(Some(Utc::now().naive_utc()));
        model.update(db)
    }
}

impl Related<super::user_link::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserLinks.def()
    }
}

impl Related<super::user_refresh_token::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RefreshToken.def()
    }
}
