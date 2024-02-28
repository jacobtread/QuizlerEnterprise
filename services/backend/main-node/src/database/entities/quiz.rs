use crate::database::DbResult;
use crate::services::auth::AuthProvider;
use chrono::Utc;
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveValue::Set, ConnectionTrait};
use serde::{Deserialize, Serialize};
use std::future::Future;

use super::user::{User, UserId};

pub type QuizId = i32;
pub type Quiz = Model;
pub type QuizEntity = Entity;
pub type QuizActiveModel = ActiveModel;

/// Database structure for a quiz
#[derive(Debug, Clone, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "quiz")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: QuizId,
    pub title: Option<String>,
    pub description: Option<String>,
    pub state: QuizState,
    pub visibility: QuizVisibility,
    pub cover_image: Option<String>,
    pub data: String,
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

impl ActiveModelBehavior for ActiveModel {}

impl Model {}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}
