use test_amiquip::consumer;

fn main() {
  let result = consumer::run_consumer();

  match result {
    Ok(()) => { println!("after run_consumer"); },
    _ => {}
  }
}