use sea_orm::DbErr;
use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error)]
pub enum DbError {
    #[error("Database connection failed: {0}")]
    ConnectionFailed(String),
    #[error("Failed to create database: {0}")]
    CreateFailed(String),
    #[error("Database operation failed: {0}")]
    OperationFailed(String),
    #[error("Serialization error: {0}")]
    SerializationError(String),
    #[error("Tauri error: {0}")]
    TauriError(String),
}

impl From<DbErr> for DbError {
    fn from(err: DbErr) -> Self {
        Self::OperationFailed(err.to_string())
    }
}

impl From<serde_json::Error> for DbError {
    fn from(err: serde_json::Error) -> Self {
        Self::SerializationError(err.to_string())
    }
}

impl From<tauri::Error> for DbError {
    fn from(err: tauri::Error) -> Self {
        Self::TauriError(err.to_string())
    }
}

/// 操作类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OperationType {
    CrawlRecord,
    CrawlPage,
    Viewed,
    Liked,
    Unliked,
    Submit,
    Create,
    Update,
    Delete,
}

impl std::fmt::Display for OperationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CrawlRecord => write!(f, "CRAWL_RECORD"),
            Self::CrawlPage => write!(f, "CRAWL_PAGE"),
            Self::Viewed => write!(f, "VIEWED"),
            Self::Liked => write!(f, "LIKED"),
            Self::Unliked => write!(f, "UNLIKED"),
            Self::Submit => write!(f, "SUBMIT"),
            Self::Create => write!(f, "CREATE"),
            Self::Update => write!(f, "UPDATE"),
            Self::Delete => write!(f, "DELETE"),
        }
    }
}

impl std::str::FromStr for OperationType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CREATE" => Ok(Self::Create),
            "UPDATE" => Ok(Self::Update),
            "DELETE" => Ok(Self::Delete),
            _ => Err(format!("Invalid operation type: {s}")),
        }
    }
}

/// 操作状态枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OperationStatus {
    Success,
    Failed,
}

impl std::fmt::Display for OperationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success => write!(f, "SUCCESS"),
            Self::Failed => write!(f, "FAILED"),
        }
    }
}

impl std::str::FromStr for OperationStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SUCCESS" => Ok(Self::Success),
            "FAILED" => Ok(Self::Failed),
            _ => Err(format!("Invalid operation status: {s}")),
        }
    }
}

/// 任务类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskType {
    Crawl,
    Submit,
    Update,
}

impl std::fmt::Display for TaskType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Crawl => write!(f, "CRAWL"),
            Self::Submit => write!(f, "SUBMIT"),
            Self::Update => write!(f, "UPDATE"),
        }
    }
}

impl std::str::FromStr for TaskType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CRAWL" => Ok(Self::Crawl),
            "SUBMIT" => Ok(Self::Submit),
            "UPDATE" => Ok(Self::Update),
            _ => Err(format!("Invalid task type: {s}")),
        }
    }
}

/// 任务状态枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    Success,
    Aborted,
    Pending,
    Failed,
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success => write!(f, "SUCCESS"),
            Self::Aborted => write!(f, "ABORTED"),
            Self::Pending => write!(f, "PENDING"),
            Self::Failed => write!(f, "FAILED"),
        }
    }
}

impl std::str::FromStr for TaskStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SUCCESS" => Ok(Self::Success),
            "ABORTED" => Ok(Self::Aborted),
            "PENDING" => Ok(Self::Pending),
            "FAILED" => Ok(Self::Failed),
            _ => Err(format!("Invalid task status: {s}")),
        }
    }
}
