use elasticsearch::{
	Error, 
	Elasticsearch, 
	http::transport::Transport, 
	cat::CatIndicesParts
};
use serde_json::Value;

pub async fn es_client_single() -> Result<(), Error> {
	let transport = Transport::single_node("http://localhost:9200")?;
	let client = Elasticsearch::new(transport);

	let response = client.cat()
		.indices(CatIndicesParts::Index(&["*"]))
		.send()
		.await?;

	let response_body = response.json::<Value>().await?;
	let as_str = response_body.as_str().unwrap();

	println!("response: {}", as_str);

	Ok(())
}