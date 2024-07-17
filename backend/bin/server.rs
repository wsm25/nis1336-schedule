
use axum::{
    extract::Request, 
    http::{header::CONTENT_TYPE, StatusCode}, 
    middleware::{from_fn, Next}, 
    response::{IntoResponse, Response}
};

use log::{info, trace};

extern crate nis1336_schedule_core as sccore;

#[tokio::main]
async fn main() {
    env_logger::init();
    // session
    let ctx = ctx::Ctx::new();

    let auth = auth::router(ctx.clone());
    let api  = api::router(ctx.clone());
    
    let app = axum::Router::new()
        .fallback(fallback)
        .nest("/auth", auth)
        .nest("/api", api)
        .layer(from_fn(session::session))
        .layer(from_fn(tracer))
        ;
    
    let port:u16 = std::env::var("SCHEDULE_PORT").map(|s| s.parse().unwrap_or(8080)).unwrap_or(8080);
    let listener = tokio::net::TcpListener::bind(("localhost", port)).await.unwrap();
    info!("Server start");
    axum::serve(listener, app).await.unwrap();
}

async fn fallback(_req: Request) -> Response {
    (
        StatusCode::NOT_FOUND, 
        [(CONTENT_TYPE, "application/json")],
        r#"{"message":"not found"}"#,
    ).into_response()
}

async fn tracer(req: Request, next: Next)->Response{
    trace!("Get Request {req:?}");
    let resp = next.run(req).await;
    trace!("Response with {resp:?}");
    resp
}

mod ctx;
mod auth;
mod api;
mod error;
mod session;