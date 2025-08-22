use crate::types::DbError;
use sea_orm::{
    ActiveModelTrait, ConnectOptions, ConnectionTrait as _, Database, DatabaseConnection,
    DbBackend, EntityTrait, Schema, Statement,
};
use tauri::{AppHandle, Manager as _};

use crate::entities::{history_op, history_task, record_local, record_remote};

pub mod impl_db;
pub mod service;

type Result<T> = std::result::Result<T, DbError>;
type Model<AM> = <<AM as ActiveModelTrait>::Entity as EntityTrait>::Model;

/// 数据库操作器
pub struct DbOperator {
    db: DatabaseConnection,
}

impl DbOperator {
    /// 初始化数据库连接
    pub async fn init(app_handle: &AppHandle) -> Result<Self> {
        // 获取应用本地数据目录 - Tauri v2 API
        let app_local_data_dir = app_handle
            .path()
            .app_local_data_dir()
            .map_err(|e| DbError::TauriError(e.to_string()))?;

        // 确保目录存在
        std::fs::create_dir_all(&app_local_data_dir)
            .map_err(|e| DbError::CreateFailed(e.to_string()))?;

        // 构建数据库文件路径
        let db_path = app_local_data_dir.join("luneth.db");
        log::debug!("Database file path: {}", db_path.display());
        let db_url = format!("sqlite://{}?mode=rwc", db_path.display());

        // 配置连接选项
        let mut opt = ConnectOptions::new(db_url);
        // 在生产环境中关闭日志
        opt.sqlx_logging(false);

        // 建立数据库连接
        let db = Database::connect(opt)
            .await
            .map_err(|e| DbError::ConnectionFailed(e.to_string()))?;

        let operator = Self { db };

        // 创建表
        operator.create_tables_if_not_exist().await?;

        Ok(operator)
    }

    /// 创建所有表（如果不存在）
    async fn create_tables_if_not_exist(&self) -> Result<()> {
        let schema = Schema::new(sea_orm::DatabaseBackend::Sqlite);
        let db_sqlite = DbBackend::Sqlite;

        // 创建 record_local 表
        let mut stmt = db_sqlite.build(&schema.create_table_from_entity(record_local::Entity));
        stmt.sql = stmt
            .sql
            .replace("CREATE TABLE", "CREATE TABLE IF NOT EXISTS");
        self.execute_db_stmt(stmt).await?;

        // 创建 record_remote 表
        let mut stmt = db_sqlite.build(&schema.create_table_from_entity(record_remote::Entity));
        stmt.sql = stmt
            .sql
            .replace("CREATE TABLE", "CREATE TABLE IF NOT EXISTS");
        self.execute_db_stmt(stmt).await?;

        // 创建 history_op 表
        let mut stmt = db_sqlite.build(&schema.create_table_from_entity(history_op::Entity));
        stmt.sql = stmt
            .sql
            .replace("CREATE TABLE", "CREATE TABLE IF NOT EXISTS");
        self.execute_db_stmt(stmt).await?;

        // 创建 history_task 表
        let mut stmt = db_sqlite.build(&schema.create_table_from_entity(history_task::Entity));
        stmt.sql = stmt
            .sql
            .replace("CREATE TABLE", "CREATE TABLE IF NOT EXISTS");
        self.execute_db_stmt(stmt).await?;

        Ok(())
    }

    /// 执行语句
    async fn execute_db_stmt(&self, stmt: Statement) -> Result<()> {
        self.db.execute(stmt).await?;
        Ok(())
    }

    pub async fn find_record_remote_by_id(&self, id: &str) -> Result<Option<record_remote::Model>> {
        let result = record_remote::Entity::find_by_id(id).one(&self.db).await?;
        Ok(result)
    }

    pub async fn find_history_task_by_id(&self, id: &str) -> Result<Option<history_task::Model>> {
        let result = history_task::Entity::find_by_id(id).one(&self.db).await?;
        Ok(result)
    }
}
