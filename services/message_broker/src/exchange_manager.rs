use crate::constants::PUB_PERSISTENT_DIR;
use axum::extract::ws::{Message, WebSocket};
use futures_util::{stream::SplitSink, SinkExt};
use queue_file::QueueFile;
use sequeda_common::exchange::Exchange;
use std::{env::var, error::Error, fmt::Display, path::PathBuf};
#[derive(Debug)]
pub struct ExchangeError {
    msg: String,
}
impl Display for ExchangeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

#[derive(Debug)]
pub struct ExchangeManager {
    subscribers: Vec<Subscriber>,
    queue: QueueFile,
}
#[derive(Debug)]
pub struct Subscriber {
    service_id: String,
    sender: SplitSink<WebSocket, Message>,
    subscriptions: Vec<String>,
}

impl ExchangeManager {
    pub fn new() -> Result<ExchangeManager, ExchangeError> {
        let path = var(PUB_PERSISTENT_DIR)
            .map(|path| PathBuf::from(&path))
            .unwrap_or_else(|_| {
                std::env::temp_dir()
                    .join("exchange_manager")
                    .join("journal")
            });
        if !path.exists() {
            std::fs::create_dir_all(&path).map_err(to_service_error)?;
        }
        if !path.is_dir() {
            return Err(ExchangeError {
                msg: format!("{path:?} not a directory"),
            });
        }
        let qf = QueueFile::open(path.join("queue.qf")).map_err(|e| ExchangeError {
            msg: format!("{e:}"),
        })?;
        Ok(Self {
            subscribers: Default::default(),
            queue: qf,
        })
    }

    pub fn connect(&mut self, service_id: &str, sender: SplitSink<WebSocket, Message>) {
        let new_subscriber = Subscriber {
            service_id: service_id.to_owned(),
            sender,
            subscriptions: vec![],
        };
        self.subscribers.push(new_subscriber);
    }

    pub fn subscribe(&mut self, service_id: &String, subscription: &str) {
        let existing_agent = self
            .subscribers
            .iter_mut()
            .find(|s| s.service_id.eq(service_id));

        if let Some(existing_subscriber) = existing_agent {
            existing_subscriber
                .subscriptions
                .push(subscription.to_uppercase());
        } else {
            tracing::info!("{service_id} not connected");
        }
    }
    pub async fn pong(&mut self, service_id: &String) -> Result<(), ExchangeError> {
        if let Some(subscriber) = self
            .subscribers
            .iter_mut()
            .find(|subscriber| subscriber.service_id.eq(service_id))
        {
            return subscriber
                .sender
                .send(Message::Pong("Pong!".into()))
                .await
                .map_err(to_service_error);
        }
        Ok(())
    }

    pub async fn close_connection(&mut self, service_id: &str) -> Result<(), ExchangeError> {
        let position = self
            .subscribers
            .iter()
            .position(|s| s.service_id.eq(service_id));
        if let Some(position) = position {
            let mut subscriber = self.subscribers.remove(position);
            subscriber.sender.close().await.map_err(to_service_error)?;
        }
        Ok(())
    }
    pub async fn consume_queue(&mut self) -> Result<(), ExchangeError> {
        let mut consumed_messages = vec![];
        for exchange_binary in self.queue.iter() {
            let mut unsubscribed = vec![];
            let exchange = Exchange::deserialize(&exchange_binary).map_err(to_service_error)?;
            let mut consumed = false;
            for (index, subscriber) in &mut self.subscribers.iter_mut().enumerate() {
                if subscriber
                    .subscriptions
                    .contains(&exchange.topic.to_uppercase())
                {
                    tracing::info!("send binary message to {}", subscriber.service_id);
                    if let Err(e) = subscriber
                        .sender
                        .send(Message::Binary(exchange_binary.to_vec()))
                        .await
                    {
                        tracing::error!("error {e} for subscriber {}", subscriber.service_id);
                        unsubscribed.push(index);
                    } else {
                        consumed = true;
                    }
                }
            }
            if consumed {
                consumed_messages.push(exchange_binary);
            }
            for position in unsubscribed {
                let mut subscriber = self.subscribers.remove(position);
                subscriber.sender.close().await.map_err(to_service_error)?;
            }
        }
        let stored = self
            .queue
            .iter()
            .filter(|ex| !consumed_messages.contains(ex))
            .map(Vec::from)
            .collect::<Vec<_>>();
        self.queue.clear().map_err(to_service_error)?;
        self.queue.add_n(stored).map_err(to_service_error)?;
        Ok(())
    }

    pub fn sync_queue_file(&mut self) -> Result<(), ExchangeError> {
        tracing::trace!("sync queue file...");
        self.queue.sync_all().map_err(to_service_error)?;
        Ok(())
    }

    pub async fn publish(&mut self, exchange_binary: Vec<u8>) -> Result<(), ExchangeError> {
        self.queue.add(&exchange_binary).map_err(to_service_error)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {

    use sequeda_common::{exchange::Exchange, TextMessage};

    #[test]
    fn make_text_message() {
        let connect = TextMessage::Connect("Nordine".into());
        let subscribe = TextMessage::Subscribe("Animal".into());

        let exchange = Exchange::new(
            r#"
        {
            "Subscribe": {
              "service_id": "Nordine",
              "topic": "Animal"
            }
          }
        "#
            .as_bytes(),
            "Animal",
            Some("topic".to_string()),
        );

        println!("{}", connect.serialize().unwrap());
        println!("{}", subscribe.serialize().unwrap());
        let bytes = bincode::serialize(&exchange).unwrap();
        let mut s: String = String::new();
        for b in bytes {
            s += &format!("{:02X}", b);
        }
        println!("{s}");
    }
}
pub fn to_service_error(e: impl Error) -> ExchangeError {
    ExchangeError { msg: e.to_string() }
}
