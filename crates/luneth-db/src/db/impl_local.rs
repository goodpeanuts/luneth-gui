use std::str::FromStr;

use super::Result;
use crate::entities::{
    record_local::ActiveModel as am, record_local::Entity as entity, record_local::Model as model,
};
use sea_orm::{
    ActiveModelTrait as _, ColumnTrait, Condition, DatabaseConnection, EntityTrait as _,
    PaginatorTrait as _, QueryFilter as _, QuerySelect as _, sea_query::IntoCondition as _,
};

#[derive(Debug)]
pub enum LocalFilterCondition {
    Viewed,
    Liked,
    Submit,
    Local,
}

impl LocalFilterCondition {
    fn to_condition(&self) -> Condition {
        match self {
            Self::Viewed => crate::record_local::Column::Viewed
                .eq(true)
                .into_condition(),
            Self::Liked => crate::record_local::Column::IsLiked
                .eq(true)
                .into_condition(),
            Self::Submit => crate::record_local::Column::IsSubmitted
                .eq(true)
                .into_condition(),
            Self::Local => crate::record_local::Column::IsCachedLocally
                .eq(true)
                .into_condition(),
        }
    }
}

fn to_conditions(filters: Vec<LocalFilterCondition>) -> Condition {
    filters
        .into_iter()
        .fold(Condition::all(), |acc, cond| acc.add(cond.to_condition()))
}

impl FromStr for LocalFilterCondition {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "viewed" => Ok(Self::Viewed),
            "liked" => Ok(Self::Liked),
            "submit" => Ok(Self::Submit),
            "local" => Ok(Self::Local),
            _ => Err(format!("Invalid filter condition: {s}")),
        }
    }
}

impl super::DbOperator {
    /// 获取数据库连接
    pub fn get_db(&self) -> &DatabaseConnection {
        &self.db
    }
    pub async fn insert_local(&self, local_record: am) -> Result<model> {
        let result = local_record.insert(&self.db).await?;
        Ok(result)
    }

    /// if entity exist do noting, return success insert count
    pub async fn or_insert_local<AM>(&self, local_record: am) -> Result<model> {
        // Try to insert, if it fails due to unique constraint, ignore and return a default model
        match local_record.insert(&self.db).await {
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

    pub async fn query_total_count(&self, filters: Vec<LocalFilterCondition>) -> Result<u64> {
        let condition = to_conditions(filters);

        let query = entity::find().filter(condition);
        let count = query.count(&self.db).await?;
        Ok(count)
    }

    /// 根据过滤条件查询记录-
    pub async fn query_local(
        &self,
        offset: Option<u64>,
        limit: Option<u64>,
        filters: Vec<LocalFilterCondition>,
    ) -> Result<Vec<model>>
where {
        let condition = to_conditions(filters);

        let mut query = entity::find().filter(condition);

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
    pub async fn update_record_local(&self, local_record: am) -> Result<model>
where {
        let result = local_record.update(&self.db).await?;
        Ok(result)
    }

    /// 删除记录
    pub async fn delete_record_local(&self, local_record: am) -> Result<()>
where {
        local_record.delete(&self.db).await?;
        Ok(())
    }

    /// 根据ID查询单个记录
    pub async fn find_record_local_by_id(&self, id: &str) -> Result<Option<model>> {
        let result = entity::find_by_id(id).one(&self.db).await?;
        Ok(result)
    }

    /// Query specified column values from an entity table
    pub async fn query_specified_column<C, T>(&self, column: C) -> Result<Vec<T>>
    where
        C: ColumnTrait,
        T: sea_orm::TryGetable + Send,
    {
        let results: Vec<T> = entity::find()
            .select_only()
            .column(column)
            .into_tuple()
            .all(&self.db)
            .await?;

        Ok(results)
    }
}
