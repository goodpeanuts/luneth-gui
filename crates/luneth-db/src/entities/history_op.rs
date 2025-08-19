use crate::types::{OperationStatus, OperationType};
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelTrait, Set};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "history_op")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub recorder_id: String,
    pub operation: String,
    pub timestamp: ChronoDateTimeUtc,
    pub status: String,
    pub user: String,
    pub error_message: Option<String>,
    pub created_at: ChronoDateTimeUtc,
    pub updated_at: ChronoDateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            timestamp: Set(chrono::Utc::now()),
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
    /// 创建新的操作历史记录
    pub fn new_record(
        recorder_id: String,
        operation: OperationType,
        status: OperationStatus,
        user: String,
        error_message: Option<String>,
    ) -> ActiveModel {
        let mut active_model = ActiveModel::new();

        active_model.recorder_id = Set(recorder_id);
        active_model.operation = Set(operation.to_string());
        active_model.status = Set(status.to_string());
        active_model.user = Set(user);
        active_model.error_message = Set(error_message);

        active_model
    }

    /// 获取操作类型枚举
    pub fn get_operation_type(&self) -> Result<OperationType, String> {
        self.operation.parse()
    }

    /// 获取操作状态枚举
    pub fn get_operation_status(&self) -> Result<OperationStatus, String> {
        self.status.parse()
    }
}
