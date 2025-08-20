use luneth::crawl::WebCrawler;

use crate::scrap::AppError;
use luneth_db::DbService;

// XXX: conditionally stop
const MAX_ITER_DEPTH: usize = 30;

pub async fn auto_crawl_page(
    db: &impl DbService,
    crawler: &WebCrawler,
    start_url: &str,
) -> Result<(), AppError> {
    for i in 0..=MAX_ITER_DEPTH {
        let url = format!("{start_url}/page/{i}");
        log::debug!("crawling page: {url}");
        let record_pieces = crawler.crawl_page(&url).await?;

        let record_ids = record_pieces
            .into_iter()
            .map(|r| r.code)
            .collect::<Vec<_>>();
        super::specified::crawl_codes(db, crawler, &record_ids).await?;
        log::debug!(
            "successfully crawled page: {url}, count: {}",
            record_ids.len()
        );
    }
    Ok(())
}
