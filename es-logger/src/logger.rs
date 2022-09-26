use elasticsearch::{
  Error,
	Elasticsearch, 
	http::transport::Transport, 
	IndexParts,
};
use serde_json::json;

pub struct Logger {
  client: Elasticsearch,
}

impl Logger {
  pub fn new(url: &str) -> Self {
    Self {
      client: Elasticsearch::new(Transport::single_node(url).unwrap())
    }
  }

  // TODO: log level, debug / trace / error
  // reconnect logic?

  pub async fn info(&self, payload: &str) -> Result<(), Error> {
    self.client.index(IndexParts::IndexId("new_key", "1"))
      .body(json!({
        "id": 1,
        "payload": payload,
      }))
      .send()
      .await?;

    Ok(())
  }
}