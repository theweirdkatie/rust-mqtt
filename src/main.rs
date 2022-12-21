use crossterm::{
    cursor::{MoveToNextLine, Show},
    event::{self, Event, KeyCode},
    terminal, ExecutableCommand,
};
use rumqtt::{MqttClient, MqttOptions, Notification, QoS, SecurityOptions};
use std::{io, thread, time::Duration};

fn main() {
    let mqtt_options = MqttOptions::new("test-pubsub1","a488e908b9e2447a8bfbedb955b8f704.s2.eu.hivemq.cloud",8883)
                        .set_ca(include_bytes!("../certs/isrgrootx1.pem").to_vec())
                        .set_security_opts(SecurityOptions::UsernamePassword("kstedman_mqtt".to_string(),"MQTTpasswordPERSONAL".to_string()))
                        .set_keep_alive(10)
                        .set_inflight(3)
                        .set_clean_session(true);

    let (mut mqtt_client, notifications) = MqttClient::start(mqtt_options).unwrap();

    let topic = "key/press";
    let mut stdout = io::stdout();
    terminal::enable_raw_mode().unwrap();
    stdout.execute(terminal::EnterAlternateScreen).unwrap();

    match mqtt_client.subscribe(topic, QoS::AtLeastOnce) {
        Ok(_) => {
            while event::poll(Duration::default()).is_ok() {
                if let Ok(Event::Key(key_event)) = event::read() {
                    match key_event.code {
                        KeyCode::Esc => break,
                        _ => {
                            // stdout.execute(terminal::Clear(terminal::ClearType::All))?;
                            let key = if let KeyCode::Char(some_char) = key_event.code {
                                format!("{some_char}")
                            } else {
                                format!("{:?}", key_event.code)
                            };
                            let msg = format!("{{\"Key pressed\": \"{}\"}}", key);
                            println!("Sending msg: {}", msg);
                            match mqtt_client.publish(topic, QoS::AtLeastOnce, false, msg) {
                                Ok(_) => {
                                    if let Some(notification) = notifications.iter().nth(0) {
                                        println!("{:?}", notification);
                                    }
                                }
                                Err(client_error) => println!("{:?}", client_error),
                            }
                        }
                    }
                }
                thread::sleep(Duration::from_millis(1));
            }
        }
        Err(client_error) => {}
    }

    stdout.execute(Show);
    terminal::disable_raw_mode().unwrap();
    stdout.execute(terminal::LeaveAlternateScreen).unwrap();
}
