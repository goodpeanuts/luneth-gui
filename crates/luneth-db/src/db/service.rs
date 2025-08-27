use super::{Model, Result};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel,
    PrimaryKeyTrait, sea_query::IntoCondition,
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
        E: EntityTrait,
        <<E as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType: for<'a> From<&'a str>;

    /// Query specified column values from an entity table
    async fn query_specified_column<E, C, T>(&self, column: C) -> Result<Vec<T>>
    where
        E: EntityTrait,
        C: ColumnTrait,
        T: sea_orm::TryGetable + Send;

    async fn get_records_ordered_by_updated_at(&self) -> Result<Vec<crate::record_local::Model>>;
}

// 使用示例：可以用通用函数查询其他列
//
// // 查询所有本地记录的标题
// let titles: Vec<String> = db.query_specified_column::<record_local::Entity, _, String>(record_local::Column::Title).await?;
//
// // 查询所有本地记录的图片数量
// let counts: Vec<i32> = db.query_specified_column::<record_local::Entity, _, i32>(record_local::Column::LocalImageCount).await?;
//
// // 查询所有远程记录的发布日期
// let dates: Vec<String> = db.query_specified_column::<record_remote::Entity, _, String>(record_remote::Column::Date).await?;
