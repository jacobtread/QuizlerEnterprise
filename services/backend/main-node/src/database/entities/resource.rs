use crate::database::DbResult;
use crate::services::auth::AuthProvider;
use chrono::Utc;
use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue::NotSet;
use sea_orm::{ActiveValue::Set, ConnectionTrait};
use serde::{Deserialize, Serialize};
use serde_json::Map;
use std::future::Future;

use super::user::{User, UserId};

pub type ResourceId = i32;
pub type Resource = Model;
pub type ResourceEntity = Entity;
pub type ResourceActiveModel = ActiveModel;

#[derive(Debug, Clone, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "quiz")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: ResourceId,
    pub mime_type: String,
    pub name: String,
    pub description: String,
    pub path: String,
    pub owner: UserId,
    pub visibility: ResourceVisibility,
    pub created_at: DateTime,
}

#[derive(Debug, Clone, Default, EnumIter, PartialEq, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum ResourceVisibility {
    #[default]
    #[sea_orm(num_value = 0)]
    Private,
    #[sea_orm(num_value = 1)]
    Public,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::Owner",
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
            self.created_at = Set(now);
        }

        Ok(self)
    }
}

impl Model {
    /// Finds a quiz by its ID
    pub fn find_by_id<C>(
        db: &C,
        id: ResourceId,
    ) -> impl Future<Output = DbResult<Option<Resource>>> + '_
    where
        C: ConnectionTrait,
    {
        Entity::find_by_id(id).one(db)
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}
