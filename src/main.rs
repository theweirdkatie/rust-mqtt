use crossterm::{
    cursor::MoveToNextLine,
    event::{self, Event, KeyCode},
    terminal, ExecutableCommand,
};
use rumqtt::{self, MqttClient, MqttOptions, QoS, SecurityOptions};
use std::{error::Error, io, time::Duration};

fn main() -> Result<(), Box<dyn Error>> {
    let host = "a488e908b9e2447a8bfbedb955b8f704.s2.eu.hivemq.cloud";

    let username = "kstedman_mqtt".to_string();
    let password = "MQTTpasswordPERSONAL".to_string();
    let topic = "test/topic";

    let conn_opts = MqttOptions::new("test-crate1", host, 8883)
        .set_security_opts(SecurityOptions::UsernamePassword(username, password))
        .set_clean_session(true);

    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    // stdout.execute(terminal::EnterAlternateScreen).unwrap();

    let result = MqttClient::start(conn_opts);

    if let Ok((mut client, notifications)) = result {
        println!("Connecting to host: '{}' at 8883", host);
        client.subscribe(topic, QoS::AtLeastOnce).unwrap();

        for notification in notifications {
            println!("{:?}", notification)
        }

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
                            let msg = format!("{{\"Key pressed\": \"{}\"}}", key);
                            println!("Sending msg: {}", msg);
                            client.publish(topic, QoS::AtLeastOnce, false, msg).unwrap();
                            stdout.execute(MoveToNextLine(1)).unwrap();
                        }
                    }
                }
            }
        }
        client.shutdown().unwrap();
    } else {
        if let Err(error) = &result {
            println!("Err: {}", error);
        }
    }
    // stdout.execute(terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode().unwrap();
    Ok(())
}
