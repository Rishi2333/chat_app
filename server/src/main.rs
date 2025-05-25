use axum::{extract::{ws::{Message, WebSocket}, State, WebSocketUpgrade}, http::HeaderValue, response::IntoResponse, routing::get, Router};
use futures_util::{SinkExt, StreamExt};
use tokio::sync::broadcast::{self, Sender};
use tower_http::cors::{Any, CorsLayer};



#[tokio::main]
async fn main() {
    let (tx, _) = broadcast::channel(100);
    let app = app(tx);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}


fn app(tx: Sender<String>) -> Router {
    let cors_layer = CorsLayer::new().allow_methods(Any).allow_origin("http://127.0.0.1:8080".parse::<HeaderValue>().unwrap());

    Router::new()
        .route("/", get(|| async {"Home"}))
        .route("/chat", get(chat_handler))
        .with_state(tx)
        .layer(cors_layer)
}

async fn chat_handler(State(tx): State<Sender<String>>, ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(|websocket| handle_websocket(tx, websocket))
}

async fn handle_websocket(tx: Sender<String>, websocket: WebSocket) {
    let (mut sender,mut receiver) = websocket.split();

    let mut rx = tx.subscribe();
    tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            sender.send(Message::from(msg)).await.unwrap()
        }
    });

    while let Some(msg) = receiver.next().await  {
        if let Ok(msg)  = msg {
            match msg {
                Message::Text(content) => {
                    tx.send(content.to_string()).unwrap();
                },
                _ => ()
            }
        }
    }
}
