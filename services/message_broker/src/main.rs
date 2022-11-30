use std::{env::var, net::SocketAddr, str::FromStr, sync::Arc, time::Duration};

use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Extension, Router,
};
use futures_util::StreamExt;
use sequeda_common::TextMessage;
use tokio::{sync::Mutex, task, time};
use tracing::Level;
use tracing_subscriber::{FmtSubscriber, EnvFilter};

use crate::{
    constants::{PUB_HOST, PUB_INTERVAL_CONSUMER, PUB_INTERVAL_SYNC_FILE, PUB_PORT},
    exchange_manager::ExchangeManager,
};

mod constants;
mod exchange_manager;


#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let app_state = Arc::new(Mutex::new(ExchangeManager::new().unwrap()));
    let app = Router::new()
        .route("/", get(ws_handler))
        .layer(Extension(app_state.clone()));
    // consume queue periodically
    let state = app_state.clone();
    let mut queue_consumer = task::spawn(async move {
        tracing::info!("starting to consume queue");
        let time_between_consume = var(PUB_INTERVAL_CONSUMER)
            .unwrap_or_else(|_| String::from("10"))
            .parse::<u64>()
            .unwrap();

        let mut interval = time::interval(Duration::from_millis(time_between_consume));
        loop {
            interval.tick().await;
            if let Err(e) = state.lock().await.consume_queue().await {
                tracing::error!("{e}");
            }
        }
    });
    let mut sync_file_task = task::spawn(async move {
        tracing::info!("interval sync queue file");
        let time_between_sync = var(PUB_INTERVAL_SYNC_FILE)
            .unwrap_or_else(|_| String::from("1000"))
            .parse::<u64>()
            .unwrap();

        let mut interval = time::interval(Duration::from_millis(time_between_sync));
        loop {
            interval.tick().await;
            if let Err(e) = app_state.lock().await.sync_queue_file() {
                tracing::error!("{e}");
            }
        }
    });
    let mut serve = tokio::spawn(async move {
        let host = var(PUB_HOST).unwrap_or_else(|_| String::from("127.0.0.1"));
        let port = var(PUB_PORT).unwrap_or_else(|_| String::from("3000"));
        let addr = SocketAddr::from_str(&format!("{host}:{port}")).unwrap();
        tracing::info!("listening on {}", addr);
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    });

    tokio::select! {
        _ = (&mut queue_consumer) => {
            serve.abort();
            sync_file_task.abort();
        },
        _ = (&mut serve) => {
            queue_consumer.abort();
            sync_file_task.abort();
        },
        _ = (&mut sync_file_task) => {
            queue_consumer.abort();
            serve.abort();
        },
    };
}
async fn ws_handler(
    ws: WebSocketUpgrade,
    Extension(state): Extension<Arc<Mutex<ExchangeManager>>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: Arc<Mutex<ExchangeManager>>) {
    let (sender, mut receiver) = socket.split();
    let task_result = tokio::spawn(async move {
        let service_id: String = {
            let mut service_id = String::new();
            while let Some(Ok(message)) = receiver.next().await {
                if let Message::Text(message) = message {
                    if let Ok(TextMessage::Connect(sid)) =
                        serde_json::from_str::<TextMessage>(&message)
                    {
                        tracing::info!("receive connect message from {sid}");
                        let mut em = state.lock().await;
                        em.connect(&sid, sender);
                        service_id = sid;
                        break;
                    }
                }
            }
            service_id
        };
        while let Some(Ok(message)) = receiver.next().await {
            match message {
                Message::Text(message) => {
                    if let Ok(TextMessage::Subscribe(topic)) =
                        serde_json::from_str::<TextMessage>(&message)
                    {
                        tracing::info!("receive subscribe message from {service_id}");
                        let mut em = state.lock().await;
                        em.subscribe(&service_id, &topic);
                    }
                }
                Message::Binary(exchange_binary) => {
                    let mut em = state.lock().await;
                    tracing::info!("receive binary message from {service_id}");
                    if let Err(e) = em.publish(exchange_binary).await {
                        tracing::error!("error in exchange {e:?}");
                    }
                }
                Message::Ping(_) => {
                    let mut em = state.lock().await;
                    if let Err(e) = em.pong(&service_id).await {
                        tracing::error!("could not send pong message {e:?}");
                    }
                }
                Message::Pong(text) => {
                    tracing::debug!("received pong message {text:?}");
                }
                Message::Close(_) => {
                    let mut em = state.lock().await;
                    if let Err(e) = em.close_connection(&service_id).await {
                        tracing::error!("could not unsubscribe {e:?}");
                    } else {
                        tracing::info!("unsubscribed");
                    }
                }
            }
        }
    })
    .await;

    if let Err(e) = task_result {
        tracing::error!("Error in task {e}");
    }
}
