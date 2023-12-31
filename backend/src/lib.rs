use chrono::{DateTime, Utc};

use async_trait::async_trait;

use serde::{Deserialize, Serialize};

#[cfg(feature = "json")]
mod json;
#[cfg(feature = "json")]
pub use json::JsonDataProvide;

#[cfg(feature = "fastapi")]
mod fastapi;
#[cfg(feature = "fastapi")]
pub use fastapi::FastApiDataProvide;

#[cfg(feature = "sqlite")]
mod sqlite;
#[cfg(feature = "sqlite")]
pub use sqlite::SqliteDataProvide;

pub const TRANSFER_DATA_VERSION: u16 = 100;

#[derive(Debug, thiserror::Error)]
pub enum ModifyEntryError {
    #[error("{0}")]
    ValidationError(String),
    #[error("{0}")]
    DataError(#[from] anyhow::Error),
}

#[async_trait]
pub trait DataProvider {
    async fn load_all_entries(&self) -> anyhow::Result<Vec<Entry>>;
    async fn add_entry(&self, entry: EntryDraft) -> Result<Entry, ModifyEntryError>;
    async fn remove_entry(&self, entry_id: u32) -> anyhow::Result<()>;
    async fn update_entry(&self, entry: Entry) -> Result<Entry, ModifyEntryError>;
    async fn get_export_object(&self, entries_ids: &[u32]) -> anyhow::Result<EntriesDTO>;
    async fn import_entries(&self, entries_dto: EntriesDTO) -> anyhow::Result<()>;
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Entry {
    pub id: u32,
    pub date: DateTime<Utc>,
    pub title: String,
    pub content: String,
    #[serde(default)]
    pub tags: Vec<String>,
}

impl Entry {
    #[allow(dead_code)]
    pub fn new(
        id: u32,
        date: DateTime<Utc>,
        title: String,
        content: String,
        tags: Vec<String>,
    ) -> Self {
        Self {
            id,
            date,
            title,
            content,
            tags,
        }
    }

    pub fn from_draft(id: u32, draft: EntryDraft) -> Self {
        Self {
            id,
            date: draft.date,
            title: draft.title,
            content: draft.content,
            tags: draft.tags,
        }
    }
}

pub struct FastApiDataProvide {
    base_url: String,
    client: reqwest::Client,
}

impl FastApiDataProvide {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: reqwest::Client::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntryDraft {
    pub date: DateTime<Utc>,
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
}

impl EntryDraft {
    pub fn new(date: DateTime<Utc>, title: String, tags: Vec<String>) -> Self {
        let content = String::new();
        Self {
            date,
            title,
            content,
            tags,
        }
    }

    pub fn from_entry(entry: Entry) -> Self {
        Self {
            date: entry.date,
            title: entry.title,
            content: entry.content,
            tags: entry.tags,
        }
    }
}

/// Entries data transfer object
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntriesDTO {
    pub version: u16,
    pub entries: Vec<EntryDraft>,
}

impl EntriesDTO {
    pub fn new(entries: Vec<EntryDraft>) -> Self {
        Self {
            version: TRANSFER_DATA_VERSION,
            entries,
        }
    }
}
