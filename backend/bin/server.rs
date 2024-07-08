use axum::{
    extract::{Request, State}, 
    http::StatusCode, 
    routing::get, 
    Json,
    response::{Response, IntoResponse}
};
use serde_json::{json, Value};
use ctx::Ctx;

#[tokio::main]
async fn main() {
    let ctx = Ctx::new();
    let app = axum::Router::new()
        .route("/count", get(count))
        .fallback(fallback)
        .layer(from_fn_with_state(ctx.clone(), counter))
        .with_state(ctx)
        ;

    let listener = tokio::net::TcpListener::bind("localhost:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn fallback(_req: Request) -> impl IntoResponse {
    (StatusCode::OK, Json(json!({
        "code": 404,
        "message": "not found",
    })))
}

async fn count(State(state): State<Ctx>) -> impl IntoResponse {
    (StatusCode::OK, Json(json!({
        "message": state.get()
    })))
}

use axum::middleware::{from_fn_with_state, Next};
async fn counter(
    State(mut state): State<Ctx>,
    request: Request,
    next: Next,
) -> Response {
    let response = next.run(request).await;
    state.inc();
    response
}

mod ctx{
    use std::{cell, sync::Arc};

    #[derive(Clone)]
    pub struct Ctx(Arc<Cell<i32>>);
    impl Ctx{
        pub fn new()->Self{
            Self(Arc::new(Cell::new(0)))
        }
        pub fn get(&self)->i32{
            self.0.get()
        }
        pub fn inc(&mut self){
            self.0.set(self.0.get()+1)
        }
    }

    struct Cell<T>(cell::Cell<T>);
    unsafe impl<T> Sync for Cell<T>{}
    impl<T:Copy+Clone> Cell<T>{
        pub fn new(inner: T)->Self{
            Self(cell::Cell::new(inner))
        }
        pub fn get(&self)->T{
            self.0.get()
        }
        pub fn set(&self, value:T){
            self.0.set(value);
        }
    }
}