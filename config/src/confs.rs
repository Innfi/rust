use config::Config;
use lazy_static::lazy_static;

#[derive(serde::Deserialize)]
pub struct TestConfigs {
  pub kvdata: KeyValueData,
}

#[derive(serde::Deserialize)]
pub struct KeyValueData {
  pub key1: String,
  pub key2: String,
}

fn load_conf() -> Result<TestConfigs, config::ConfigError> {
  let base_path = std::env::current_dir().expect("dir error");
  let config_dir = base_path.join("confs");
  let config_file = config::File::from(config_dir.join("base"));

  let config = Config::builder().add_source(config_file).build().unwrap();

  config.try_deserialize()
}

lazy_static! {
  pub static ref CONFS: TestConfigs = load_conf().unwrap();
}