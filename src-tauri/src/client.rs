use std::sync::LazyLock;

use luneth::{client::Postman, common::RecordSlimDto};
use tokio::sync::Mutex;

#[derive(Clone, Debug)]
pub struct ClientAuth {
    pub url: String,
    pub id: String,
    pub secret: String,
}

pub static CLIENT_AUTH: LazyLock<Mutex<Option<ClientAuth>>> = LazyLock::new(|| {
    Mutex::new(None) // Default base URL
});

pub async fn pull_record_slim() -> anyhow::Result<Vec<RecordSlimDto>> {
    let Some(auth) = CLIENT_AUTH.lock().await.clone() else {
        anyhow::bail!("Client authentication is not set");
    };
    let mut client = Postman::new(&auth.url, auth.id, auth.secret);

    let records = client.pull_remote_records_slim().await?;

    Ok(records)
}
