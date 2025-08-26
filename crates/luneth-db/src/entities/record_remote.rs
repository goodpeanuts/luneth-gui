use luneth::common::RecordSlimDto;
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelTrait, Set};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "record_remote")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,

    pub title: String,

    pub date: String,

    pub duration: i32,

    pub director: String,

    pub studio: String,

    pub label: String,

    pub series: String,

    #[sea_orm(column_type = "Json")]
    pub genres: Json,

    #[sea_orm(column_type = "Json")]
    pub idols: Json,

    pub has_links: bool,

    #[sea_orm(column_type = "Json")]
    pub links: Json,

    pub created_at: ChronoDateTimeUtc,
    pub updated_at: ChronoDateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            id: Set(uuid::Uuid::new_v4().to_string()),
            genres: Set(Json::Array(vec![])),
            idols: Set(Json::Array(vec![])),
            has_links: Set(false),
            links: Set(Json::Array(vec![])),
            created_at: Set(chrono::Utc::now()),
            updated_at: Set(chrono::Utc::now()),
            ..ActiveModelTrait::default()
        }
    }

    fn before_save<'life0, 'async_trait, C>(
        mut self,
        _db: &'life0 C,
        _insert: bool,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<Self, DbErr>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        C: 'async_trait + ConnectionTrait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            self.updated_at = Set(chrono::Utc::now());
            Ok(self)
        })
    }
}

impl From<RecordSlimDto> for ActiveModel {
    fn from(dto: RecordSlimDto) -> Self {
        Self {
            id: Set(dto.id),
            title: Set(dto.title),
            date: Set(dto.date),
            duration: Set(dto.duration),
            director: Set(dto.director),
            studio: Set(dto.studio),
            label: Set(dto.label),
            series: Set(dto.series),
            genres: Set(serde_json::to_value(&dto.genres).unwrap_or(Json::Array(vec![]))),
            idols: Set(serde_json::to_value(&dto.idols).unwrap_or(Json::Array(vec![]))),
            has_links: Set(dto.has_links),
            links: Set(serde_json::to_value(&dto.links).unwrap_or(Json::Array(vec![]))),
            created_at: Set(chrono::Utc::now()),
            updated_at: Set(chrono::Utc::now()),
        }
    }
}

impl Model {
    pub fn from_record_slim_dto(dto: RecordSlimDto) -> ActiveModel {
        ActiveModel::from(dto)
    }
}
