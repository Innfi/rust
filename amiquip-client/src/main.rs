use test_amiquip::consumer;

/**
 * how to recover from failure (both queue and program itself)?
 * 
 */

fn main() {
  let result = consumer::run_consumer();

  match result {
    Ok(()) => { println!("after run_consumer"); },
    _ => {}
  }
}