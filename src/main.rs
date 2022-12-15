//! This is a simple asynchronous MQTT publisher using SSL/TSL secured
//! connection via the Paho MQTT Rust Library.

 use futures::executor::block_on;
 use paho_mqtt as mqtt;
 use std::{env, process};
 
 /////////////////////////////////////////////////////////////////////////////
 
 fn main() -> mqtt::Result<()> {
     // Initialize the logger from the environment
     env_logger::init();
 
     // We use the trust store from the Paho C tls-testing/keys directory,
     // but we assume there's a copy in the current directory.
     const TRUST_STORE: &str = "/certs/certificate.pem.crt";
     const KEY_STORE: &str = "/certs/AmazonRootCA1.pem";
 
     // We assume that we are in a valid directory.
     let mut trust_store = env::current_dir()?;
     trust_store.push(TRUST_STORE);
 
     let mut key_store = env::current_dir()?;
     key_store.push(KEY_STORE);
 
     if !trust_store.exists() {
         println!("The trust store file does not exist: {:?}", trust_store);
         println!("  Get a copy from \"paho.mqtt.c/test/ssl/{}\"", TRUST_STORE);
         process::exit(1);
     }
 
     if !key_store.exists() {
         println!("The key store file does not exist: {:?}", key_store);
         println!("  Get a copy from \"paho.mqtt.c/test/ssl/{}\"", KEY_STORE);
         process::exit(1);
     }
 
     // Let the user override the host, but note the "ssl://" protocol.
     let host = env::args()
         .nth(1)
         .unwrap_or_else(|| "https://a33dx7mb3rdv3r-ats.iot.us-west-2.amazonaws.com:8883".to_string());
 
     println!("Connecting to host: '{}'", host);
 
     // Run the client in an async block
 
     if let Err(err) = block_on(async {
         // Create a client & define connect options
         let cli = mqtt::CreateOptionsBuilder::new()
             .server_uri(&host)
             .client_id("ssl_publish_rs")
             .max_buffered_messages(100)
             .create_client()?;
 
         let ssl_opts = mqtt::SslOptionsBuilder::new()
             .trust_store(trust_store)?
             .key_store(key_store)?
             .finalize();
 
         let conn_opts = mqtt::ConnectOptionsBuilder::new()
             .ssl_options(ssl_opts)
             .finalize();
 
         cli.connect(conn_opts).await?;
 
         let msg = mqtt::MessageBuilder::new()
             .topic("test")
             .payload("Hello secure world!")
             .qos(1)
             .finalize();
 
         cli.publish(msg).await?;
         cli.disconnect(None).await?;
 
         Ok::<(), mqtt::Error>(())
     }) {
         eprintln!("{}", err);
     }
     Ok(())
 }
 