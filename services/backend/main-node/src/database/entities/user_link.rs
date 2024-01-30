use crate::database::DbResult;
use crate::services::auth::AuthProvider;
use chrono::Utc;
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveValue::Set, ConnectionTrait};
use std::future::Future;

use super::user::{User, UserId};

pub type UserLink = Model;
pub type UserLinkEntity = Entity;
pub type UserLinkActiveModel = ActiveModel;

/// Database structure for a user
#[derive(Debug, Clone, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "user_links")]
pub struct Model {
    /// Unique ID for the user
    #[sea_orm(primary_key)]
    pub user_id: UserId,
    #[sea_orm(primary_key)]
    pub provider: AuthProvider,
    /// When this user was created
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

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    /// Create a new user link
    pub fn create<'db, C>(
        db: &'db C,
        user: &User,
        provider: AuthProvider,
    ) -> impl Future<Output = DbResult<u64>> + 'db
    where
        C: ConnectionTrait,
    {
        Entity::insert(ActiveModel {
            user_id: Set(user.id),
            provider: Set(provider),
            created_at: Set(Utc::now().naive_utc()),
        })
        .exec_without_returning(db)
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
