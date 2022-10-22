use std::time::Duration;
use std::{env::var, error::Error, fmt::Display};

use futures_util::{SinkExt, StreamExt};
use sequeda_common::exchange::Exchange;
use sequeda_common::TextMessage;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{self, http::Request},
    MaybeTlsStream, WebSocketStream,
};

pub const MSG_CONS_HOST: &str = "MSG_CONS_HOST";
pub const MSG_CONS_PORT: &str = "MSG_CONS_PORT";
pub const MSG_CONS_PROTOCOL: &str = "MSG_CONS_PROTOCOL";
pub const MSG_CONS_TIMEOUT: &str = "MSG_CONS_TIMEOUT";

#[derive(Debug)]
pub struct MessageClient {
    _agent: String,
    _socket: WebSocketStream<MaybeTlsStream<TcpStream>>,
    _timeout: Duration,
}
#[derive(Debug)]
pub struct MessageClientError {
    msg: String,
}

impl Display for MessageClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}
pub fn to_lib_error(e: impl Error) -> MessageClientError {
    MessageClientError { msg: e.to_string() }
}

impl MessageClient {
    pub async fn new(agent: &str) -> Result<MessageClient, MessageClientError> {
        let host = var(MSG_CONS_HOST).unwrap_or_else(|_| String::from("127.0.0.1"));
        let port = var(MSG_CONS_PORT).unwrap_or_else(|_| String::from("3000"));
        let protocol = var(MSG_CONS_PROTOCOL).unwrap_or_else(|_| String::from("ws"));
        let timeout = var(MSG_CONS_TIMEOUT)
            .unwrap_or_else(|_| String::from("1000"))
            .parse::<u64>()
            .map_err(to_lib_error)?;
        let url = format!("{protocol}://{host}:{port}");
        let request = Request::builder()
            .method("GET")
            .header("Host", &url)
            .header("Connection", "Upgrade")
            .header("Upgrade", "websocket")
            .header("Sec-WebSocket-Version", "13")
            .header(
                "Sec-WebSocket-Key",
                tungstenite::handshake::client::generate_key(),
            )
            .uri(&url)
            .body(())
            .map_err(to_lib_error)?;
        let (mut ws_stream, _) = connect_async(request).await.map_err(to_lib_error)?;
        let connect = TextMessage::Connect(agent.into());

        ws_stream
            .send(tungstenite::Message::Text(
                connect.serialize().map_err(to_lib_error)?,
            ))
            .await
            .map_err(to_lib_error)?;
        Ok(MessageClient {
            _agent: agent.into(),
            _socket: ws_stream,
            _timeout: Duration::from_millis(timeout),
        })
    }

    pub async fn subscribe(&mut self, topic: &str) -> Result<(), MessageClientError> {
        let subscribe = TextMessage::Subscribe(topic.into());
        self._socket
            .send(tungstenite::Message::Text(
                subscribe.serialize().map_err(to_lib_error)?,
            ))
            .await
            .map_err(to_lib_error)?;
        Ok(())
    }
    pub async fn recv(&mut self) -> Option<Result<Exchange, MessageClientError>> {
        tracing::info!("receiving...");

        if let Ok(Some(msg)) = tokio::time::timeout(self._timeout, self._socket.next()).await {
            match msg {
                Ok(tungstenite::Message::Binary(binary)) => {
                    Some(Exchange::deserialize(&binary).map_err(to_lib_error))
                }
                message => {
                    tracing::error!("socket sent an invalid message {message:?}");
                    None
                }
            }
        } else {
            None
        }
    }

    pub async fn send(&mut self, message: Exchange) -> Result<(), MessageClientError> {
        let msg = message.serialize().map_err(to_lib_error)?;
        self._socket
            .send(Message::Binary(msg))
            .await
            .map_err(to_lib_error)?;
        Ok(())
    }
}

impl Drop for MessageClient {
    fn drop(&mut self) {
        futures::executor::block_on(async move {
            self._socket.close(None).await.unwrap();
        })
    }
}

#[cfg(test)]
mod tests {}
