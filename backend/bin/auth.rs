// todo: remove unused session id

use axum::{
    extract::State, 
    response::Response, 
    routing::post, Extension, Json
};
use serde::Deserialize;
use crate::{
    ctx::Ctx,
    session::SessionID,
    error::{Result, Error::*},
};

pub fn router(ctx: Ctx)->axum::Router {
    // todo: logout
    axum::Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
        // todo: cors layer
        .with_state(ctx)
}

#[derive(Deserialize)]
struct UserAuth {
    username: String,
    password: String,
}

async fn login (
    State(ctx): State<Ctx>,
    Extension(id): Extension<SessionID>,
    Json(ua): Json<UserAuth>,
) -> Result<Response> 
{
    if ua.username.is_empty() 
        {return Err(InvalidUser);}
    ctx.login(&id, &ua.username, &ua.password)?;
    Err(OK)
}

async fn register (
    State(ctx): State<Ctx>,
    Extension(id): Extension<SessionID>,
    Json(ua): Json<UserAuth>,
) -> Result<Response> 
{
    if ua.username.is_empty() 
        {return Err(InvalidUser)}
    if ua.password.len()<8 
        {return Err(PwdTooShort);}
    ctx.register(&id, &ua.username, &ua.password)?;
    Err(OK)
}