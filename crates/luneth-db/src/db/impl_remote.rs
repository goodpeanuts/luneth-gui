use super::Result;
use crate::entities::record_remote;
use sea_orm::ActiveModelTrait as _;

impl super::DbOperator {
    pub async fn insert_remote(
        &self,
        remote_am: record_remote::ActiveModel,
    ) -> Result<record_remote::Model> {
        let model = remote_am.insert(&self.db).await?;
        Ok(model)
    }
}
