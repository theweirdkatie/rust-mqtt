//! This is a simple asynchronous MQTT publisher using SSL/TSL secured
//! connection via the Paho MQTT Rust Library.

use crossterm::{
    event::{self, Event, KeyCode},
    terminal, ExecutableCommand,
};
use futures::executor::block_on;
use paho_mqtt as mqtt;
use std::{io, time::Duration};

/////////////////////////////////////////////////////////////////////////////

fn main() -> mqtt::Result<()> {
    // Initialize the logger from the environment
    env_logger::init();

    const ROOT_CA_PATH: &str = "certs/AmazonRootCA1.pem";
    const CERTIFICATE_PATH: &str = "certs/certificate.pem.crt";
    const PRIVATE_KEY_PATH: &str = "certs/private.pem.key";
    const ALPN_PROTOCOLS: &[&str] = &["x-amzn-mqtt-ca"];

    let host = "ssl://a33dx7mb3rdv3r-ats.iot.us-west-2.amazonaws.com:443".to_string();

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
            .trust_store(ROOT_CA_PATH)?
            .key_store(CERTIFICATE_PATH)?
            .private_key(PRIVATE_KEY_PATH)?
            .alpn_protos(ALPN_PROTOCOLS)
            .enable_server_cert_auth(false)
            .verify(true)
            .finalize();

        let conn_opts = mqtt::ConnectOptionsBuilder::new()
            .clean_session(true)
            .ssl_options(ssl_opts)
            .finalize();

        let rsp = cli.connect(conn_opts).await.expect("could not connect");

        let mut stdout = io::stdout();
        terminal::enable_raw_mode()?;
        stdout.execute(terminal::EnterAlternateScreen)?;

        'msg_loop: loop {
            while event::poll(Duration::default())? {
                if let Event::Key(key_event) = event::read()? {
                    match key_event.code {
                        KeyCode::Esc => break 'msg_loop,
                        _ => {
                            stdout.execute(terminal::Clear(terminal::ClearType::All))?;
                            let key = if let KeyCode::Char(some_char) = key_event.code {
                                format!("{some_char}")
                            } else {
                                format!("{:?}", key_event.code)
                            };
                            let msg = mqtt::MessageBuilder::new()
                                .topic("test")
                                .payload(format!("{{\"Key pressed\": \"{}\"}}", key))
                                .qos(1)
                                .finalize();
                            println!("Sending msg: {}", msg);
                            cli.publish(msg).await?;
                        }
                    }
                }
            }
        }
        cli.disconnect(None).await?;
        stdout.execute(terminal::LeaveAlternateScreen)?;

        Ok::<(), mqtt::Error>(())
    }) {
        eprintln!("{}", err);
    }
    Ok(())
}
