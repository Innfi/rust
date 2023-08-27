use futures::{pin_mut, TryStreamExt};
use k8s_openapi::api::{core::v1::ObjectReference, events::v1::Event};
use kube::{
  api::Api,
  runtime::{watcher, WatchStreamExt},
  Client,
};
use tracing::info;

async fn run_event_watch() -> anyhow::Result<()> {
  tracing_subscriber::fmt::init();
  let client = Client::try_default().await?;

  let event_api: Api<Event> = Api::all(client);
  let event_watcher = watcher(event_api, watcher::Config::default()).applied_objects();

  // pin_mut!(event_watcher); // not getting what this macro means
  while let Some(event) = event_watcher.try_next().await? {
    handle_event(event)?;
  }

  Ok(())
}

fn handle_event(ev: Event) -> Result<()> {
  info!("reason: {}, note: {}",
    ev.reason.unwrap_or_default(),
    ev.not.unwrap_or_default());

  Ok(())
}