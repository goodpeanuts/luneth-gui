use std::fmt::Debug;

use luneth::crawl::WebCrawler;

use crate::db::{log_failed_crawl_op, log_success_crawl_op};
use crate::scrap::AppError;
use luneth_db::entities::record_local::Model as RecorderModel;
use luneth_db::DbService;

pub async fn crawl_codes<T>(
    db: &impl DbService,
    crawler: &WebCrawler,
    codes: &[T],
) -> Result<(), AppError>
where
    T: AsRef<str> + Debug,
{
    for code in codes {
        let code = code.as_ref();
        log::debug!("Crawling code: {code}");

        match crawler.crawl_code(code).await {
            Ok(record) => {
                let record_model = RecorderModel::from_recorder(&record);
                let insert_result = db.insert_entity(record_model).await;

                match insert_result {
                    Ok(_) => log_success_crawl_op(db, code).await?,
                    Err(e) => {
                        log::error!("Failed to insert record for code {code}: {e}");
                        log_failed_crawl_op(db, code, e.to_string()).await?;
                    }
                };
            }
            Err(e) => {
                log::error!("Failed to crawl code {code}: {e}");
                log_failed_crawl_op(db, code, e.to_string()).await?;
            }
        }
    }
    Ok(())
}
