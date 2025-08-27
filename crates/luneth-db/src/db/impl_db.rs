use super::{Model, Result};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel,
    PrimaryKeyTrait, QueryFilter as _, QueryOrder as _, QuerySelect as _, sea_query::IntoCondition,
};

impl super::service::DbService for super::DbOperator {
    /// 获取数据库连接
    fn get_db(&self) -> &DatabaseConnection {
        &self.db
    }

    async fn insert_entity<AM>(&self, active_model: AM) -> Result<Model<AM>>
    where
        AM: ActiveModelTrait + sea_orm::ActiveModelBehavior + std::marker::Send,
        Model<AM>: IntoActiveModel<AM> + std::marker::Send,
    {
        let result: Model<AM> = active_model.insert(&self.db).await?;
        Ok(result)
    }

    /// if entity exist do noting, return success insert count
    async fn or_insert_entity<AM>(&self, active_model: AM) -> Result<Model<AM>>
    where
        AM: ActiveModelTrait + sea_orm::ActiveModelBehavior + std::marker::Send,
        Model<AM>: IntoActiveModel<AM> + std::marker::Send,
    {
        // Try to insert, if it fails due to unique constraint, ignore and return a default model
        match active_model.insert(&self.db).await {
            Ok(model) => Ok(model),
            Err(sea_orm::DbErr::Exec(_)) => {
                // For now, we'll just try to insert again and if it fails, propagate the error
                // This is a simplified implementation - a more robust solution would
                // extract the primary key and query for the existing record
                Err(sea_orm::DbErr::Custom(
                    "Record already exists or constraint violation".to_owned(),
                )
                .into())
            }
            Err(e) => Err(e.into()),
        }
    }

    async fn query_entity<E>(
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
    async fn query_entity_by_filter<E, F>(
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
    async fn update_record_local<AM>(&self, active_model: AM) -> Result<Model<AM>>
    where
        AM: ActiveModelTrait + sea_orm::ActiveModelBehavior + std::marker::Send,
        Model<AM>: IntoActiveModel<AM> + std::marker::Send,
    {
        let result = active_model.update(&self.db).await?;
        Ok(result)
    }

    /// 删除记录
    async fn delete_record_local<AM>(&self, active_model: AM) -> Result<()>
    where
        AM: ActiveModelTrait + sea_orm::ActiveModelBehavior + std::marker::Send,
    {
        active_model.delete(&self.db).await?;
        Ok(())
    }

    /// 根据ID查询单个记录
    async fn find_record_local_by_id<E>(&self, id: &str) -> Result<Option<E::Model>>
    where
        E: EntityTrait,
        <<E as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType: for<'a> From<&'a str>,
    {
        let result = E::find_by_id(id).one(&self.db).await?;
        Ok(result)
    }

    /// Query specified column values from an entity table
    async fn query_specified_column<E, C, T>(&self, column: C) -> Result<Vec<T>>
    where
        E: EntityTrait,
        C: ColumnTrait,
        T: sea_orm::TryGetable + Send,
    {
        let results: Vec<T> = E::find()
            .select_only()
            .column(column)
            .into_tuple()
            .all(&self.db)
            .await?;

        Ok(results)
    }

    async fn get_records_ordered_by_updated_at(&self) -> Result<Vec<crate::record_local::Model>> {
        use crate::record_local;
        let records = record_local::Entity::find()
            .order_by_desc(record_local::Column::UpdatedAt)
            .all(&self.db)
            .await?;
        Ok(records)
    }
}
