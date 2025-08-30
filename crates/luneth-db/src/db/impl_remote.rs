use super::Result;
use crate::entities::record_remote;
use sea_orm::{ActiveModelTrait as _, EntityTrait as _, QuerySelect as _};

impl super::DbOperator {
    pub async fn insert_remote(
        &self,
        remote_am: record_remote::ActiveModel,
    ) -> Result<record_remote::Model> {
        let model = remote_am.insert(&self.db).await?;
        Ok(model)
    }

    /// Query specified column values from an entity table
    pub async fn query_remote_specified_column<C, T>(&self, column: C) -> Result<Vec<T>>
    where
        C: sea_orm::ColumnTrait,
        T: sea_orm::TryGetable + Send,
    {
        use crate::entities::record_remote::Entity as entity;
        let results: Vec<T> = entity::find()
            .select_only()
            .column(column)
            .into_tuple()
            .all(&self.db)
            .await?;

        Ok(results)
    }
}
