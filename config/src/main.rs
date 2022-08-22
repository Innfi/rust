use test_confs::confs::CONFS;

fn main() {
  println!("Hello, world!");

  println!("key1: {}, key2: {}", CONFS.kvdata.key1, CONFS.kvdata.key2);
}
