use core::time;
use std::sync::{Arc, Mutex};
use std::thread;


fn main() {
  let counter = Arc::new(Mutex::new(0));
  let cloned = Arc::clone(&counter);

  thread::spawn(move || {
    loop {
      thread::sleep(time::Duration::from_millis(10000));

      let mut lock = cloned.try_lock();
      if let Ok(ref mut mutex) = lock {
        println!("before reset: {}", **mutex);
        **mutex = 0;
        continue;
      }

      println!("spawned] try_lock failed");
    }
  });

  loop {
    thread::sleep(time::Duration::from_millis(1000));

    let mut lock = counter.try_lock();
    if let Ok(ref mut mutex) = lock {
      println!("before increment: {}", **mutex);
      **mutex += 1;
      continue;
    } 

    println!("try_lock failed");
  }
}

// fn main() {
//   let counter = Arc::new(Mutex::new(0));
//   let mut handles = vec![];
// 
//   for _ in 0..10 {
//     let counter = Arc::clone(&counter);
//     let handle = thread::spawn(move || {
//       let mut num = counter.lock().unwrap();
// 
//       *num += 1;
//     });
// 
//     handles.push(handle);
//   }
// 
//   for handle in handles {
//     handle.join().unwrap();
//   }
// 
//   println!("result: {}", *counter.lock().unwrap());
// }