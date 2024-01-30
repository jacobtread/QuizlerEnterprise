use crate::database::DbResult;
use chrono::Utc;
use sea_orm::sea_query::OnConflict;
use sea_orm::{entity::prelude::*, ActiveValue};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ConnectionTrait};
use std::future::Future;

use super::user::{User, UserId};

pub type UserRefreshToken = Model;
pub type UserRefreshTokenEntity = Entity;
pub type UserRefreshTokenActiveModel = ActiveModel;

/// Database structure for a user
#[derive(Debug, Clone, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "user_refresh_tokens")]
pub struct Model {
    /// The ID of the user the token belongs to
    #[sea_orm(unique)]
    pub user_id: UserId,
    /// The refresh token itself
    #[sea_orm(primary_key)]
    pub refresh_token: String,
    /// When this refresh token was created
    pub created_at: DateTime,
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
        let now = Utc::now().naive_utc();
        if insert {
            self.created_at = ActiveValue::Set(now);
        }

        Ok(self)
    }
}

impl Model {
    /// Create a new refresh token for the provided `user`
    pub fn create<'db, C>(
        db: &'db C,
        user: &User,
        refresh_token: String,
    ) -> impl Future<Output = DbResult<UserRefreshToken>> + 'db
    where
        C: ConnectionTrait,
    {
        Entity::insert(ActiveModel {
            user_id: Set(user.id),
            refresh_token: Set(refresh_token),
            created_at: Set(Utc::now().naive_utc()),
        })
        .on_conflict(
            OnConflict::column(Column::UserId)
                .update_columns([Column::RefreshToken, Column::CreatedAt])
                .to_owned(),
        )
        .exec_with_returning(db)
    }

    /// Find a refresh token by token
    pub fn find_by_token<'db, C>(
        db: &'db C,
        refresh_token: &str,
    ) -> impl Future<Output = DbResult<Option<UserRefreshToken>>> + 'db
    where
        C: ConnectionTrait,
    {
        Entity::find_by_id(refresh_token).one(db)
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}
