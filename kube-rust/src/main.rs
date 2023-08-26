use k8s_openapi::api::core::v1::Pod;
use serde_json::json;
use tracing::*;

use kube::{
  api::{Api, DeleteParams, ListParams, Patch, PatchParams, PostParams, ResourceExt}, 
  runtime::wait::{await_condition, conditions::is_pod_running},
  Client,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  tracing_subscriber::fmt::init();
  let client = Client::try_default().await?;

  // let pods: Api<Pod> = Api::default_namespaced(client);
  let pod_api: Api<Pod> = Api::namespaced(client, "innfi");

  info!("creating test pod");
  let pod: Pod = serde_json::from_value(json!({
    "apiVersion": "vi",
    "kind": "pod",
    "metadata": { "name": "test_blog"},
    "spec": {
      "containers": [{
        "name": "blog",
        "image": "clux/blog:0.1.0",
      }],
    },
  }))?;

  let pp = PostParams::default();
  match pod_api.create(&pp, &pod).await {
    Ok(o) => {
      let name = o.name_any();
      assert_eq!(pod.name_any, name);
      info!("created {}", name)
    },
    Err(kube::Error::Api(apiErr)) => {
      error!("pod creation error: {}", api);
      assert_eq!(apiErr.code, 409)
    },
    Err(e) => return Err(e.into()),
  }

  let establish = await_condition(pod_api.clone(), "test_blog", is_pod_running());
  let _ = tokio::time::timeout(std::time::Duration::from_secs(15), establish).await?;

  let pod_copy = pod_api.get("test_blog").await?;
  if let Some(spec) = &pod_copy.spec {
    info!("pod container: {:?}", spec.containers);
    assert_eq!(spec.containers[0].name, "test_blog")
  }

  // change spec
  // delete pod

  Ok(())
}