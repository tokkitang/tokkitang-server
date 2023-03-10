use std::sync::Arc;
use std::time::Duration;
use std::{collections::HashMap, error::Error};

use aws_sdk_dynamodb::Client;
use axum::body::{Body, BoxBody};
use axum::middleware::from_fn;
use axum::{
    http::{HeaderMap, Request, Response, StatusCode},
    middleware,
    response::{Html, IntoResponse},
    routing::get,
    Extension, Json, Router,
};
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::{Level, Span};

use crate::extensions::{CurrentUser, DynamoClient, S3Client};

use crate::middlewares::{auth_middleware, response_header_middleware};
use crate::routes::{auth, entity, note, project, redirect, team, user, utils};
use crate::utils::send_email;

pub(crate) async fn router() -> Router {
    let trace = TraceLayer::new_for_http()
        .on_request(|request: &Request<Body>, _span: &Span| {
            println!("{} {} started", request.method(), request.uri().path());
            println!("request: {request:?}",);
        })
        .on_response(
            |response: &Response<BoxBody>, latency: Duration, _span: &Span| {
                println!("response generated in {latency:?}",);
                println!("response: {response:?}",);
            },
        );

    Router::new()
        .route("/", get(index))
        .route("/health", get(health))
        .nest("/utils", utils::router().await)
        .nest("/user", user::router().await)
        .nest("/auth", auth::router().await)
        .nest("/redirect", redirect::router().await)
        .nest("/team", team::router().await)
        .nest("/project", project::router().await)
        .nest("/note", note::router().await)
        .nest("/entity", entity::router().await)
        .route_layer(from_fn(response_header_middleware))
        .route_layer(middleware::from_fn(auth_middleware))
        .layer(Extension(DynamoClient::get_client().await))
        .layer(Extension(S3Client::get_client().await))
        .layer(trace)
}

async fn index() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

use super::dto::health_response::HealthReponse;

async fn health(current_user: Extension<CurrentUser>) -> impl IntoResponse {
    let server_ok = true;
    let authorized = current_user.authorized;

    Json(HealthReponse {
        server_ok,
        authorized,
    })
    .into_response()
}
