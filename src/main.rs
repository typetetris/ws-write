use axum::{
    routing::get,
    Router, extract::{Path, ws::{self, WebSocket}}, response::IntoResponse, http::HeaderMap,
};
use tokio::{io::AsyncWriteExt, fs};
use tracing_bunyan_formatter::BunyanFormattingLayer;
use tracing_subscriber::{Registry, layer::SubscriberExt};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // initialize tracing
    let formatting_layer = BunyanFormattingLayer::new("tracing_demo".into(), std::io::stdout);
    let subscriber = Registry::default().with(formatting_layer);
    tracing::subscriber::set_global_default(subscriber).unwrap();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/*path", get(root));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 7777));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root(Path(path): Path<String>, ws: ws::WebSocketUpgrade, headers: HeaderMap) -> impl IntoResponse {
    let sane_path = format!("./{}", path.replace('/', "_"));
    let mut open_options = fs::OpenOptions::new();
    let handle = open_options.write(true).truncate(true).create(true).open(sane_path).await.unwrap();

    for (k,v) in headers.iter() {
        tracing::debug!("{}: {:#?}", k, v);
    }

    ws.on_upgrade(move |socket| {
        async {
            handle_socket(socket, handle).await
        }
    })
}

async fn handle_socket(mut socket: WebSocket, mut handle: fs::File) {
    while let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            match msg {
                ws::Message::Text(msg) => {handle.write_all(msg.as_bytes()).await.unwrap();},
                ws::Message::Binary(msg) => {handle.write_all(&msg).await.unwrap();},
                ws::Message::Ping(p) => {socket.send(ws::Message::Pong(p)).await.unwrap();},
                ws::Message::Pong(_) => { continue },
                ws::Message::Close(_) => { continue; },
            }
        } else {
            handle.flush().await.unwrap();
            // client disconnected
            return;
        };
    }
}
