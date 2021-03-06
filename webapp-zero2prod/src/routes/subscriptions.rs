use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;
use tracing::Instrument;

#[derive(serde::Deserialize)]
pub struct FormData {
  email: String,
  name: String,
}

#[tracing::instrument(
  name = "adding a subscriber",
  skip(form, pool),
  fields(
    subscriber_email = %form.email,
    subscriber_name = %form.name,
  )
)]
pub async fn subscribe(
  form: web::Form<FormData>,
  pool: web::Data<PgPool>
) -> HttpResponse {

  let subscriber_name = crate::domain::SubscriberName(form.name.clone());

  let query_span = tracing::info_span!(
    "saving new subscriber details in the database"
  );
  match sqlx::query!(
    r#"
    INSERT INTO subscriptions (id, email, name, subscribed_at) 
    VALUES ($1, $2, $3, $4)
    "#,
    Uuid::new_v4(),
    form.email,
    form.name,
    Utc::now()
  )
  .execute(pool.get_ref())
  .instrument(query_span)
  .await
  {
    Ok(_) => {
      HttpResponse::Ok().finish()
    },
    Err(e) => {
      tracing::error!("failed to execute query: {:?}", e);
      HttpResponse::InternalServerError().finish()
    }
  }
}
