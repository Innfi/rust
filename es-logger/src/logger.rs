use elasticsearch::{
  Error,
	Elasticsearch, 
	http::transport::Transport, 
	IndexParts,
};
use serde_json::json;
use serde::{Serialize, Deserialize};
// use chrono::prelude::*;

pub struct Logger {
  index: String,
  client: Elasticsearch,
}

#[derive(Serialize, Deserialize)]
pub struct LogPayload {
  pub time: i64,
  pub log_str: String,
}

impl Logger {
  pub fn new(index_name: &str, url: &str) -> Self {
    Self {
      index: index_name.into(),
      client: Elasticsearch::new(Transport::single_node(url).unwrap())
    }
  }

  // TODO: log level, debug / trace / error
  // reconnect logic?

  pub async fn info(&self, payload: LogPayload) -> Result<(), Error> {
    self.client.index(IndexParts::IndexId(self.index.as_str(), "1"))
      .body(json!(payload))
      .send()
      .await?;

    Ok(())
  }
}