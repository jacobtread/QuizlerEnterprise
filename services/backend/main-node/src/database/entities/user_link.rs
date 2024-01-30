use crate::database::DbResult;
use crate::services::auth::AuthProvider;
use chrono::Utc;
use sea_orm::IntoActiveModel;
use sea_orm::{entity::prelude::*, ActiveValue};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ConnectionTrait};
use std::future::Future;

use super::user::User;

pub type UserLink = Model;
pub type UserLinkEntity = Entity;
pub type UserLinkActiveModel = ActiveModel;

/// Database structure for a user
#[derive(Debug, Clone, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "user_links")]
pub struct Model {
    /// Unique ID for the user
    #[sea_orm(primary_key)]
    pub user_id: u32,
    #[sea_orm(primary_key)]
    pub provider: AuthProvider,
    /// When this user was created
    pub created_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
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
        if insert {
            self.created_at = ActiveValue::Set(now);
        }

        Ok(self)
    }
}

impl Model {
    /// Create a new user
    pub fn create<'db, C>(
        db: &'db C,
        user: &User,
        provider: AuthProvider,
    ) -> impl Future<Output = DbResult<UserLink>> + 'db
    where
        C: ConnectionTrait,
    {
        ActiveModel {
            user_id: Set(user.id),
            provider: Set(provider),
            ..Default::default()
        }
        .insert(db)
    }

    /// Finds a link to the provided `provider` for the provided `user`
    /// if one exists
    pub fn find_by_user<'db, C>(
        db: &'db C,
        user: &User,
        provider: AuthProvider,
    ) -> impl Future<Output = DbResult<Option<UserLink>>> + 'db
    where
        C: ConnectionTrait,
    {
        user.find_related(Entity)
            .filter(Column::Provider.eq(provider))
            .one(db)
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}
