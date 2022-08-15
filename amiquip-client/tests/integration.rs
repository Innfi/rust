use test_amiquip;
//use test_amiquip::publisher;
use test_amiquip::consumer;

#[test]
fn test_pubsub() {
  let result = consumer::run_consumer();

  match result {
    Ok(()) => {},
    _ => { panic!("invalid result"); }
  }
}