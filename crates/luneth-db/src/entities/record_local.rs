use luneth::record::{RecordEntry, Recorder};
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelTrait, IntoActiveModel as _, Set};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "record_local")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub title: String,
    pub release_date: String,
    pub length: String,
    #[sea_orm(column_type = "Json")]
    pub director: Json,
    #[sea_orm(column_type = "Json")]
    pub studio: Json,
    #[sea_orm(column_type = "Json")]
    pub label: Json,
    #[sea_orm(column_type = "Json")]
    pub series: Json,
    #[sea_orm(column_type = "Json")]
    pub genre: Json,
    #[sea_orm(column_type = "Json")]
    pub idols: Json,
    #[sea_orm(column_type = "Json")]
    pub share_magnet_links: Json,
    pub local_image_count: i32,

    /// Cover image URL
    pub cover: String,

    #[sea_orm(column_type = "Json")]
    pub sample_image_links: Json,

    pub viewed: bool,
    pub is_liked: bool,
    pub is_submitted: bool,
    pub is_cached_locally: bool,

    pub created_at: ChronoDateTimeUtc,
    pub updated_at: ChronoDateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            id: Set(uuid::Uuid::new_v4().to_string()),
            local_image_count: Set(0),
            director: Set(Json::Object(serde_json::Map::new())),
            studio: Set(Json::Object(serde_json::Map::new())),
            label: Set(Json::Object(serde_json::Map::new())),
            series: Set(Json::Object(serde_json::Map::new())),
            genre: Set(Json::Object(serde_json::Map::new())),
            idols: Set(Json::Object(serde_json::Map::new())),
            share_magnet_links: Set(Json::Array(vec![])),
            sample_image_links: Set(Json::Array(vec![])),
            created_at: Set(chrono::Utc::now()),
            updated_at: Set(chrono::Utc::now()),

            is_liked: Set(false),
            is_submitted: Set(false),
            is_cached_locally: Set(false),
            viewed: Set(false),
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

impl Model {
    pub fn from_recorder(recorder: &Recorder) -> ActiveModel {
        let mut active_model = ActiveModel::new();

        active_model.id = Set(recorder.record.id.clone());
        active_model.title = Set(recorder.record.title.clone());
        active_model.release_date = Set(recorder.record.release_date.clone());
        active_model.length = Set(recorder.record.length.clone());
        active_model.local_image_count = Set(recorder.record.local_image_count);

        // 转换 HashMap 为 JSON
        active_model.director =
            Set(serde_json::to_value(&recorder.record.director).unwrap_or_default());
        active_model.studio =
            Set(serde_json::to_value(&recorder.record.studio).unwrap_or_default());
        active_model.label = Set(serde_json::to_value(&recorder.record.label).unwrap_or_default());
        active_model.series =
            Set(serde_json::to_value(&recorder.record.series).unwrap_or_default());
        active_model.genre = Set(serde_json::to_value(&recorder.record.genre).unwrap_or_default());
        active_model.idols = Set(serde_json::to_value(&recorder.record.idols).unwrap_or_default());

        // 转换数组为 JSON
        active_model.share_magnet_links =
            Set(serde_json::to_value(&recorder.record.share_magnet_links).unwrap_or_default());

        active_model.cover = Set(recorder.cover.clone());
        active_model.sample_image_links =
            Set(serde_json::to_value(&recorder.sample_image_links).unwrap_or_default());

        active_model
    }

    pub fn from_recorder_with_image_local(recorder: &Recorder) -> ActiveModel {
        let mut active_model = Self::from_recorder(recorder);

        // 标记为本地缓存
        active_model.is_cached_locally = Set(true);

        active_model
    }

    pub fn into_record(&self) -> RecordEntry {
        RecordEntry {
            id: self.id.clone(),
            title: self.title.clone(),
            release_date: self.release_date.clone(),
            length: self.length.clone(),
            director: serde_json::from_value(self.director.clone()).unwrap_or_default(),
            studio: serde_json::from_value(self.studio.clone()).unwrap_or_default(),
            label: serde_json::from_value(self.label.clone()).unwrap_or_default(),
            series: serde_json::from_value(self.series.clone()).unwrap_or_default(),
            genre: serde_json::from_value(self.genre.clone()).unwrap_or_default(),
            idols: serde_json::from_value(self.idols.clone()).unwrap_or_default(),
            share_magnet_links: serde_json::from_value(self.share_magnet_links.clone())
                .unwrap_or_default(),
            local_image_count: self.local_image_count,
        }
    }

    pub fn set_viewd(self, viewed: bool) -> ActiveModel {
        let mut active_model = self.into_active_model();
        active_model.viewed = Set(viewed);
        active_model
    }

    pub fn set_liked(self, liked: bool) -> ActiveModel {
        let mut active_model = self.into_active_model();
        active_model.is_liked = Set(liked);
        active_model
    }
}
