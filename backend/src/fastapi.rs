use reqwest;
use std::collections::HashMap;
use super::{DataProvider, Entry, EntryDraft, ModifyEntryError, EntriesDTO};
use serde::{Deserialize, Serialize};
use async_trait::async_trait;

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

#[async_trait]
impl DataProvider for FastApiDataProvide {
    async fn load_all_entries(&self) -> anyhow::Result<Vec<Entry>> {
        let resp = self.client.get(format!("{}/api-endpoint", self.base_url))
            .send().await?
            .json::<Vec<Entry>>().await?;
        Ok(resp)
    }

    async fn add_entry(&self, entry: EntryDraft) -> Result<Entry, ModifyEntryError> {
        let resp = self.client.post(format!("{}/api-endpoint", self.base_url))
            .json(&entry)
            .send().await?
            .json::<Entry>().await?;
        Ok(resp)
    }

    // Implement the other methods similarly
}