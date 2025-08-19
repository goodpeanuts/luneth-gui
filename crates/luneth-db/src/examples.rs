use std::collections::HashMap;
use luneth_db::*;

/// 使用示例
pub async fn example_usage() -> Result<()> {
    // 假设我们已经有了一个 AppHandle
    // let app_handle = ...; 
    
    // 1. 初始化数据库
    // let db = DbOperator::init(&app_handle).await?;
    
    // 2. 创建一个示例 Recorder
    let mut recorder = Recorder {
        url: "https://example.com/video/abc123".to_string(),
        html: "<html><body>Example HTML</body></html>".to_string(),
        document: scraper::Html::parse_document("<html><body>Example HTML</body></html>"),
        base_url: "https://example.com".to_string(),
        cover: "https://example.com/cover.jpg".to_string(),
        title: "Example Video Title".to_string(),
        id: "abc123".to_string(),
        release_date: "2024-01-01".to_string(),
        length: "120min".to_string(),
        director: HashMap::from([("Director Name".to_string(), "director_link".to_string())]),
        studio: HashMap::from([("Studio Name".to_string(), "studio_link".to_string())]),
        label: HashMap::new(),
        series: HashMap::new(),
        genre: HashMap::from([("Action".to_string(), "action_link".to_string())]),
        idols: HashMap::from([("Actor Name".to_string(), "actor_link".to_string())]),
        share_magnet_links: vec![
            MagnetLink {
                name: "High Quality".to_string(),
                link: "magnet:?xt=urn:btih:...".to_string(),
                size: "1.5GB".to_string(),
            }
        ],
        sample_image_links: vec!["https://example.com/sample1.jpg".to_string()],
        local_image_count: 0,
    };
    
    // 3. 插入 record_local
    // let active_model = record_local::Model::from_recorder(&recorder);
    // let inserted = db.insert_record_local(active_model).await?;
    // println!("Inserted record: {:?}", inserted);
    
    // 4. 查询 record_local
    // let records = db.query_record_local(Some(0), Some(10)).await?;
    // println!("Found {} records", records.len());
    
    // 5. 根据ID查找记录
    // let found_record = db.find_record_local_by_id("abc123").await?;
    // if let Some(record) = found_record {
    //     println!("Found record: {}", record.title);
    // }
    
    // 6. 创建操作历史记录
    // let history = history_op::Model::new_record(
    //     "abc123".to_string(),
    //     OperationType::Create,
    //     OperationStatus::Success,
    //     "user1".to_string(),
    //     None,
    // );
    // let history_result = db.insert_history_op(history).await?;
    // println!("Created history: {:?}", history_result);
    
    // 7. 创建任务记录
    // let task = history_task::Model::new_task(
    //     TaskType::Crawl,
    //     TaskStatus::Pending,
    //     vec!["abc123".to_string(), "def456".to_string()],
    // );
    // let task_result = db.insert_history_task(task).await?;
    // println!("Created task: {:?}", task_result);
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_recorder_conversion() {
        let recorder = Recorder {
            url: "https://test.com".to_string(),
            html: "<html></html>".to_string(),
            document: scraper::Html::parse_document("<html></html>"),
            base_url: "https://test.com".to_string(),
            cover: "cover.jpg".to_string(),
            title: "Test Title".to_string(),
            id: "test123".to_string(),
            release_date: "2024-01-01".to_string(),
            length: "60min".to_string(),
            director: HashMap::new(),
            studio: HashMap::new(),
            label: HashMap::new(),
            series: HashMap::new(),
            genre: HashMap::new(),
            idols: HashMap::new(),
            share_magnet_links: vec![],
            sample_image_links: vec![],
            local_image_count: 0,
        };
        
        // 测试转换为 ActiveModel
        let active_model = crate::entities::record_local::Model::from_recorder(&recorder);
        assert!(active_model.id.is_set());
        
        // 这里可以添加更多测试...
    }
}
