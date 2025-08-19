use crate::types::DbError;
use sea_orm::{
    ActiveModelTrait, ConnectOptions, ConnectionTrait as _, Database, DatabaseConnection,
    DbBackend, EntityTrait, IntoActiveModel, PrimaryKeyTrait, QueryFilter as _, QuerySelect as _,
    Schema, Statement, sea_query::IntoCondition,
};
use tauri::{AppHandle, Manager as _};

use crate::entities::{history_op, history_task, record_local, record_remote};

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
        let db_url = format!("sqlite://{}?mode=rwc", db_path.display());

        // 配置连接选项
        let mut opt = ConnectOptions::new(db_url);
        // 在生产环境中关闭日志
        opt.sqlx_logging(false);

        // 建立数据库连接
        let db = Database::connect(opt)
            .await
            .map_err(|e| DbError::ConnectionFailed(e.to_string()))?;

        let mut operator = Self { db };

        // 创建表
        operator.create_tables_if_not_exist().await?;

        Ok(operator)
    }

    /// 创建所有表（如果不存在）
    async fn create_tables_if_not_exist(&mut self) -> Result<()> {
        let schema = Schema::new(sea_orm::DatabaseBackend::Sqlite);
        let db_sqlite = DbBackend::Sqlite;

        // 创建 record_local 表
        let stmt = db_sqlite.build(&schema.create_table_from_entity(record_local::Entity));
        self.execute_db_stmt(stmt).await?;

        // 创建 record_remote 表
        let stmt = db_sqlite.build(&schema.create_table_from_entity(record_remote::Entity));
        self.execute_db_stmt(stmt).await?;

        // 创建 history_op 表
        let stmt = db_sqlite.build(&schema.create_table_from_entity(history_op::Entity));
        self.execute_db_stmt(stmt).await?;

        // 创建 history_task 表
        let stmt = db_sqlite.build(&schema.create_table_from_entity(history_task::Entity));
        self.execute_db_stmt(stmt).await?;

        Ok(())
    }

    /// 执行语句
    async fn execute_db_stmt(&self, stmt: Statement) -> Result<()> {
        self.db.execute(stmt).await?;
        Ok(())
    }

    /// 获取数据库连接
    pub fn get_db(&self) -> &DatabaseConnection {
        &self.db
    }

    pub async fn insert_entity<AM>(&self, active_model: AM) -> Result<Model<AM>>
    where
        AM: ActiveModelTrait + sea_orm::ActiveModelBehavior + std::marker::Send,
        Model<AM>: IntoActiveModel<AM> + std::marker::Send,
    {
        let result: Model<AM> = active_model.insert(&self.db).await?;
        Ok(result)
    }

    pub async fn query_entity<E>(
        &self,
        offset: Option<u64>,
        limit: Option<u64>,
    ) -> Result<Vec<E::Model>>
    where
        E: EntityTrait,
    {
        let mut query = E::find();

        if let Some(offset) = offset {
            query = query.offset(offset);
        }

        if let Some(limit) = limit {
            query = query.limit(limit);
        }

        let results = query.all(&self.db).await?;
        Ok(results)
    }

    /// 根据过滤条件查询记录
    pub async fn query_entity_by_filter<E, F>(
        &self,
        filter: F,
        offset: Option<u64>,
        limit: Option<u64>,
    ) -> Result<Vec<E::Model>>
    where
        E: EntityTrait,
        F: IntoCondition,
    {
        let mut query = E::find().filter(filter);

        if let Some(offset) = offset {
            query = query.offset(offset);
        }

        if let Some(limit) = limit {
            query = query.limit(limit);
        }

        let results = query.all(&self.db).await?;
        Ok(results)
    }

    /// 更新记录
    pub async fn update_record_local<AM>(&self, active_model: AM) -> Result<Model<AM>>
    where
        AM: ActiveModelTrait + sea_orm::ActiveModelBehavior + std::marker::Send,
        Model<AM>: IntoActiveModel<AM> + std::marker::Send,
    {
        let result = active_model.update(&self.db).await?;
        Ok(result)
    }

    /// 删除记录
    pub async fn delete_record_local<AM>(&self, active_model: AM) -> Result<()>
    where
        AM: ActiveModelTrait + sea_orm::ActiveModelBehavior + std::marker::Send,
    {
        active_model.delete(&self.db).await?;
        Ok(())
    }

    /// 根据ID查询单个记录
    pub async fn find_record_local_by_id<E>(&self, id: &str) -> Result<Option<E::Model>>
    where
        E: EntityTrait + Into<<E::PrimaryKey as PrimaryKeyTrait>::ValueType>,
        <<E as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType: for<'a> From<&'a str>,
    {
        let result = E::find_by_id(id).one(&self.db).await?;
        Ok(result)
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
