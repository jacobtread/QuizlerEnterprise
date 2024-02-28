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

pub type QuizId = i32;
pub type Quiz = Model;
pub type QuizEntity = Entity;
pub type QuizActiveModel = ActiveModel;

/// Database structure for a quiz
#[derive(Debug, Clone, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "quiz")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: QuizId,
    pub title: String,
    pub description: String,
    pub state: QuizState,
    pub visibility: QuizVisibility,
    pub cover_image: Option<String>,
    pub data: serde_json::Value,
    pub owner: UserId,
    /// When this quiz was created
    pub created_at: DateTime,
    /// When this quiz was updated
    pub updated_at: DateTime,
}

#[derive(Debug, Clone, Default, EnumIter, PartialEq, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum QuizState {
    #[default]
    #[sea_orm(num_value = 0)]
    Draft,
    #[sea_orm(num_value = 1)]
    Published,
}

#[derive(Debug, Clone, Default, EnumIter, PartialEq, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum QuizVisibility {
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
        self.updated_at = Set(now);

        if insert {
            self.created_at = Set(now);
        }

        Ok(self)
    }
}

impl Model {
    /// Create a new quiz
    pub fn create<'db, C>(
        db: &'db C,
        owner: &User,
        title: String,
    ) -> impl Future<Output = DbResult<Quiz>> + 'db
    where
        C: ConnectionTrait,
    {
        ActiveModel {
            title: Set(title),
            description: Set(String::new()),
            state: Set(QuizState::Draft),
            visibility: Set(QuizVisibility::Private),
            cover_image: Set(None),
            data: Set(serde_json::Value::Object(Map::new())),
            owner: Set(owner.id),
            ..Default::default()
        }
        .insert(db)
    }

    /// Finds a quiz by its ID
    pub fn find_by_id<C>(db: &C, id: QuizId) -> impl Future<Output = DbResult<Option<Quiz>>> + '_
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
