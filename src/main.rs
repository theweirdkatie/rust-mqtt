use rumqtt::{MqttClient, MqttOptions, QoS, SecurityOptions};
use std::{thread, time::Duration};

fn main() {
    let mqtt_options = MqttOptions::new("test-pubsub1", "a488e908b9e2447a8bfbedb955b8f704.s2.eu.hivemq.cloud", 8883)
                        .set_ca(include_bytes!("../certs/isrgrootx1.pem").to_vec())
                        .set_security_opts(SecurityOptions::UsernamePassword("".to_string(), "".to_string()))
                        .set_keep_alive(10)
                        .set_inflight(3)
                        .set_clean_session(true);

    let (mut mqtt_client, notifications) = MqttClient::start(mqtt_options).unwrap();
      
    mqtt_client.subscribe("hello/world", QoS::AtLeastOnce).unwrap();
    let sleep_time = Duration::from_secs(1);
    thread::spawn(move || {
        for i in 0..100 {
            let payload = format!("publish {}", i);
            thread::sleep(sleep_time);
            mqtt_client.publish("hello/world", QoS::AtLeastOnce, false, payload).unwrap();
        }
    });

    for notification in notifications {
        println!("{:?}", notification)
    }
}