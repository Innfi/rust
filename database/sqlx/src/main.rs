use sqlx::MySqlPool;


pub struct DatabaseManager {
  connection_pool: MySqlPool,
}

impl DatabaseManager {
  pub async fn new() -> Self {
    Self {
      connection_pool: MySqlPool::connect(&"mysql://localhost:3306/test").await.expect("failed to connect"),
    }
  }

  pub async fn select_data(&self, id: &i32) -> Result<i32, &'static str> {
    let select_result = sqlx::query!(
      r#"SELECT id from dictionary WHERE id=?;"#,
      id,
    )
    .fetch_one(&self.connection_pool)
    .await;

    Ok(select_result.id)
  }

  pub async fn insert_data(&self, id: &i32) -> Result<(), &'static str> {
    let isnert_result = sqlx::query!(
      r#"INSERT INTO dictionary(id) VALUES (?);"#,
      id,
    )
    .execute(&self.connection_pool)
    .await;

    Ok(())
  }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
  let manager = DatabaseManager::new().await;

  let _ = manager.insert_data(&3).await.expect("insert failed");

  let select_result = manager.select_data(&3).expect("select failed");

  Ok(())
}
