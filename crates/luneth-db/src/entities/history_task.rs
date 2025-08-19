use crate::types::{TaskStatus, TaskType};
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelTrait, Set};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "history_task")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub task_type: String,
    pub start_time: ChronoDateTimeUtc,
    pub end_time: Option<ChronoDateTimeUtc>,
    pub status: String,
    #[sea_orm(column_type = "Json")]
    pub target_ids: Json,
    #[sea_orm(column_type = "Json")]
    pub failed_ids: Json,
    pub total_count: i32,
    pub failed_count: i32,
    pub created_at: ChronoDateTimeUtc,
    pub updated_at: ChronoDateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            id: Set(uuid::Uuid::new_v4().to_string()),
            start_time: Set(chrono::Utc::now()),
            target_ids: Set(Json::Array(vec![])),
            failed_ids: Set(Json::Array(vec![])),
            total_count: Set(0),
            failed_count: Set(0),
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

impl Model {
    /// 创建新的任务记录
    pub fn new_task(
        task_type: TaskType,
        status: TaskStatus,
        target_ids: Vec<String>,
    ) -> ActiveModel {
        let mut active_model = ActiveModel::new();

        active_model.task_type = Set(task_type.to_string());
        active_model.status = Set(status.to_string());
        active_model.target_ids = Set(serde_json::to_value(&target_ids).unwrap_or_default());
        active_model.total_count = Set(target_ids.len() as i32);

        active_model
    }

    /// 更新任务状态
    pub fn update_status(&self, status: TaskStatus, failed_ids: Vec<String>) -> ActiveModel {
        let mut active_model: ActiveModel = self.clone().into();

        active_model.status = Set(status.to_string());
        active_model.failed_ids = Set(serde_json::to_value(&failed_ids).unwrap_or_default());
        active_model.failed_count = Set(failed_ids.len() as i32);

        if matches!(
            status,
            TaskStatus::Success | TaskStatus::Failed | TaskStatus::Aborted
        ) {
            active_model.end_time = Set(Some(chrono::Utc::now()));
        }

        active_model
    }

    /// 获取任务类型枚举
    pub fn get_task_type(&self) -> Result<TaskType, String> {
        self.task_type.parse()
    }

    /// 获取任务状态枚举
    pub fn get_task_status(&self) -> Result<TaskStatus, String> {
        self.status.parse()
    }

    /// 获取目标ID列表
    pub fn get_target_ids(&self) -> Result<Vec<String>, serde_json::Error> {
        serde_json::from_value(self.target_ids.clone())
    }

    /// 获取失败ID列表
    pub fn get_failed_ids(&self) -> Result<Vec<String>, serde_json::Error> {
        serde_json::from_value(self.failed_ids.clone())
    }
}
