use amiquip::{
  Connection, 
  ConsumerMessage, 
  ConsumerOptions, 
  QueueDeclareOptions, 
  Result
};

pub fn run_consumer() -> Result<()> {
  let mut connection = Connection::insecure_open("amqp://localhost:5672")?;
  let channel = connection.open_channel(None)?;
  let queue = channel.queue_declare(
      "first_queue", 
      QueueDeclareOptions::default()
  )?;

  let consumer = queue.consume(ConsumerOptions::default())?;
  println!("waiting for messages");

  for (i, message) in consumer.receiver().iter().enumerate() {
      match message {
          ConsumerMessage::Delivery(delivery) => {
              let body = String::from_utf8_lossy(&delivery.body);
              println!("({:>3}) received: [{}]", i, body);
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