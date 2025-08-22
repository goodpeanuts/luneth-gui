use super::{Model, Result};
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, EntityTrait, IntoActiveModel, PrimaryKeyTrait,
    sea_query::IntoCondition,
};

#[expect(async_fn_in_trait)]
pub trait DbService {
    fn get_db(&self) -> &DatabaseConnection;

    async fn insert_entity<AM>(&self, active_model: AM) -> Result<Model<AM>>
    where
        AM: ActiveModelTrait + sea_orm::ActiveModelBehavior + std::marker::Send,
        Model<AM>: IntoActiveModel<AM> + std::marker::Send;

    async fn or_insert_entity<AM>(&self, active_model: AM) -> Result<Model<AM>>
    where
        AM: ActiveModelTrait + sea_orm::ActiveModelBehavior + std::marker::Send,
        Model<AM>: IntoActiveModel<AM> + std::marker::Send;

    async fn query_entity<E>(
        &self,
        offset: Option<u64>,
        limit: Option<u64>,
    ) -> Result<Vec<E::Model>>
    where
        E: EntityTrait;

    async fn query_entity_by_filter<E, F>(
        &self,
        filter: F,
        offset: Option<u64>,
        limit: Option<u64>,
    ) -> Result<Vec<E::Model>>
    where
        E: EntityTrait,
        F: IntoCondition;

    async fn update_record_local<AM>(&self, active_model: AM) -> Result<Model<AM>>
    where
        AM: ActiveModelTrait + sea_orm::ActiveModelBehavior + std::marker::Send,
        Model<AM>: IntoActiveModel<AM> + std::marker::Send;

    async fn delete_record_local<AM>(&self, active_model: AM) -> Result<()>
    where
        AM: ActiveModelTrait + sea_orm::ActiveModelBehavior + std::marker::Send;

    async fn find_record_local_by_id<E>(&self, id: &str) -> Result<Option<E::Model>>
    where
        E: EntityTrait + Into<<E::PrimaryKey as PrimaryKeyTrait>::ValueType>,
        <<E as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType: for<'a> From<&'a str>;
}
