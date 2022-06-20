use amiquip::{Connection, Exchange, Publish, Result};

pub fn run_publisher() -> Result<()> {
  let mut connection = Connection::insecure_open("amqp://localhost:5672")?;
  let channel = connection.open_channel(None)?;
  let exchange = Exchange::direct(&channel);

  
  exchange.publish(Publish::new("test".as_bytes(), "first_queue"))?;

  connection.close()
}