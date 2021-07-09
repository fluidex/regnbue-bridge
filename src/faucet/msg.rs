use crate::mq::messages::WrappedMessage;
use crate::mq::msg_consumer::{Simple, SimpleConsumer, SimpleMessageHandler};
use rdkafka::consumer::StreamConsumer;
use rdkafka::message::{BorrowedMessage, Message};

const UNIFY_TOPIC: &str = "unifyevents";
const MSG_TYPE_USERS: &str = "registeruser";

pub fn load_msgs_from_mq(
    brokers: &str,
    sender: crossbeam_channel::Sender<WrappedMessage>,
) -> Option<std::thread::JoinHandle<anyhow::Result<()>>> {
    let consumer: StreamConsumer = rdkafka::config::ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("group.id", "faucet_msg_consumer")
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .set("auto.offset.reset", "earliest")
        .create()
        .unwrap();
    let writer = MessageWriter { sender };

    Some(std::thread::spawn(move || {
        let rt: tokio::runtime::Runtime = tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap();
        rt.block_on(async move {
            let consumer = std::sync::Arc::new(consumer);
            let cr_main = SimpleConsumer::new(consumer.as_ref())
                .add_topic(UNIFY_TOPIC, Simple::from(&writer))
                .unwrap();
            tokio::select! {
                err = cr_main.run_stream(|cr|cr.stream()) => {
                    log::error!("Kafka consumer error: {}", err);
                }
            }
        });

        Ok(())
    }))
}

struct MessageWriter {
    sender: crossbeam_channel::Sender<WrappedMessage>,
}

impl SimpleMessageHandler for &MessageWriter {
    fn on_message(&self, msg: &BorrowedMessage<'_>) {
        let msg_type = std::str::from_utf8(msg.key().unwrap()).unwrap();
        if msg_type != MSG_TYPE_USERS {
            return;
        };

        let msg_payload = std::str::from_utf8(msg.payload().unwrap()).unwrap();
        let data = serde_json::from_str(msg_payload).unwrap();
        let message = WrappedMessage::User(data);
        self.sender.try_send(message).unwrap();
    }
}
