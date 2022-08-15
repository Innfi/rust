use amiquip::{
  Connection, 
  ConsumerMessage, 
  ConsumerOptions, 
  QueueDeclareOptions, 
  Result
};

pub fn message_handler(seq: usize, payload: &Vec<u8>) {
  let body = String::from_utf8_lossy(payload);

  println!("({:>3}) received: [{}]", seq, body);
}

pub fn run_consumer() -> Result<()> {
  let mut connection = Connection::insecure_open("amqp://localhost:5672")?;
  let channel = connection.open_channel(None).expect("open_channel failed");
  let queue = channel.queue_declare(
      "first_queue", 
      QueueDeclareOptions::default()
  ).expect("queue_declare failed");

  let consumer = queue.consume(ConsumerOptions::default())?;
  println!("waiting for messages");

  for (i, message) in consumer.receiver().iter().enumerate() {
      match message {
          ConsumerMessage::Delivery(delivery) => {
              message_handler(i, &delivery.body);
              consumer.ack(delivery)?;
          },
          other => {
              println!("consumer ended: {:?}", other);
              break;
          }
      }
  }

  connection.close()
}