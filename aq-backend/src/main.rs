use std::{collections::HashMap, sync::atomic::{AtomicUsize, Ordering}};

use axum::{extract::{Query, State}, http::StatusCode, response::IntoResponse, routing::get, serve, BoxError, Router};
use axum_macros::debug_handler;
use socketioxide::{
    extract::SocketRef,
    SocketIo,
};

use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing::{info, instrument};


#[instrument]
async fn on_connect(socket: SocketRef) {
    info!("New connection: {:?}", socket);
}

#[tokio::main]
async fn main() -> Result<(), BoxError> {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let (layer, io) = SocketIo::builder().build_layer();

    io.ns("/", on_connect);

    // build our application with a route
    let app = Router::new()
        // .with_state(room_store.clone())
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
                .layer(layer),
        );

    info!("Starting server...");

    // run our app with hyper, listening globally on port 80
    let listener = tokio::net::TcpListener::bind("0.0.0.0:80").await?;
    serve(listener, app).await?;

    Ok(())
}

