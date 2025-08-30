use sea_orm::{ActiveModelTrait as _, EntityTrait as _};

use super::Result;
use crate::history_op;

impl super::DbOperator {
    pub async fn query_history_op(&self) -> Result<Vec<history_op::Model>> {
        let history = history_op::Entity::find().all(&self.db).await?;
        Ok(history)
    }

    pub async fn insert_history_op(
        &self,
        history_op: history_op::ActiveModel,
    ) -> Result<history_op::Model> {
        let model = history_op.insert(&self.db).await?;
        Ok(model)
    }
}
