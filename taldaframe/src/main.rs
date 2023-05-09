use std::{sync::{Arc, RwLock}, collections::HashMap, borrow::Cow};

use axum::{
    routing::get,
    Router, response::{IntoResponse, Response}, extract::{Path, State, DefaultBodyLimit, Query, self}, body::{Bytes, Body}, http::{StatusCode, Request}, BoxError, handler::Handler, middleware::{Next, self},
};
use base64::{Engine as Base64Engine, engine::general_purpose};
use serde::Deserialize;
use tower_http::{compression::CompressionLayer, limit::{RequestBodyLimitLayer}};
use wasmtime::{Engine, Module, Store, Instance};

type SharedState = Arc<RwLock<AppState>>;

#[derive(Default)]
struct WasmApp {
    wasm_bytecode: Bytes,
    options: Option<WasmAppOptions>,
}

#[derive(Default)]
struct AppState {
    wasm_apps: HashMap<String, WasmApp>,
}

#[derive(Debug, Deserialize)]
struct WasmAppOptions {
    wasi: bool,
}

#[derive(Debug, Deserialize)]
struct WasmAppSetParams {
    wasm_bytecode: String,
    options: Option<WasmAppOptions>,
}

async fn wasm_app_get(
    Path(key): Path<String>,
    State(state): State<SharedState>,
) -> Result<String, StatusCode> {
    let wasm_apps = &state.read().unwrap().wasm_apps;

    if let Some(value) = wasm_apps.get(&key) {
        Ok(general_purpose::STANDARD_NO_PAD.encode(&value.wasm_bytecode))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

async fn wasm_app_set(Path(key): Path<String>, State(state): State<SharedState>, extract::Json(params): extract::Json<WasmAppSetParams>) -> impl IntoResponse {
    let wasm_bytecode = general_purpose::STANDARD_NO_PAD.decode(&mut Bytes::from(params.wasm_bytecode));
    if let Err(e) = wasm_bytecode {
        println!("wasm_app_set: {:?}", e);
        return (StatusCode::BAD_REQUEST, "Invalid base64".to_string());
    }
    let wasm_bytecode = wasm_bytecode.unwrap();
    state.write().unwrap().wasm_apps.insert(key, WasmApp {
        wasm_bytecode: wasm_bytecode.into(),
        options: params.options,
    });

    (StatusCode::OK, "OK".to_string())
}

async fn wasm_app_execute(
    Path(key): Path<String>,
    State(state): State<SharedState>,
) -> impl IntoResponse {
    let wasm_apps = &state.read().unwrap().wasm_apps;

    if let Some(value) = wasm_apps.get(&key) {
        let engine = Engine::default();
        let module = Module::new(&engine, &value.wasm_bytecode);
        if let Err(e) = module {
            println!("wasm_app_execute: {:?}", e);
            return Err(StatusCode::BAD_REQUEST);
        }
        let module = module.unwrap();
        let mut store = Store::new(&engine, ());
        let instance = Instance::new(&mut store, &module, &[]).unwrap();
        let run = instance.get_typed_func::<(), u32>(&mut store, "run");
        if let Err(e) = run {
            println!("wasm_app_execute: {:?}", e);
            return Err(StatusCode::BAD_REQUEST);
        }
        let run = run.unwrap();

        let result = run.call(&mut store, ());
        if let Err(e) = result {
            println!("wasm_app_execute: {:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
        let result = result.unwrap();

        Ok(result.to_string())
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

#[tokio::main]
async fn main() {
    let shared_state = SharedState::default();

    // build our application with a single route
    let app = Router::new().route(
        "/wasm/:key", 
        get(wasm_app_get.layer(CompressionLayer::new()))
        .post_service(
            wasm_app_set
                .layer((
                    DefaultBodyLimit::disable(), 
                    RequestBodyLimitLayer::new(1024 * 1024 * 10),
                ))
                .with_state(Arc::clone(&shared_state)),
        ),
    ).route(
        "/wasm/:key/execute", 
        get(wasm_app_execute)
    )
    .with_state(Arc::clone(&shared_state));
    // .layer(middleware::from_fn(print_request_response));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handle_error(error: BoxError) -> impl IntoResponse {
    if error.is::<tower::timeout::error::Elapsed>() {
        return (StatusCode::REQUEST_TIMEOUT, Cow::from("request timed out"));
    }

    if error.is::<tower::load_shed::error::Overloaded>() {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Cow::from("service is overloaded, try again later"),
        );
    }

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Cow::from(format!("Unhandled internal error: {}", error)),
    )
}

async fn print_request_response(
    req: Request<Body>,
    next: Next<Body>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let (parts, body) = req.into_parts();
    let bytes = buffer_and_print("request", body).await?;
    let req = Request::from_parts(parts, Body::from(bytes));

    let res = next.run(req).await;

    let (parts, body) = res.into_parts();
    let bytes = buffer_and_print("response", body).await?;
    let res = Response::from_parts(parts, Body::from(bytes));

    Ok(res)
}

async fn buffer_and_print<B>(direction: &str, body: B) -> Result<Bytes, (StatusCode, String)>
where
    B: axum::body::HttpBody<Data = Bytes>,
    B::Error: std::fmt::Display,
{
    let bytes = match hyper::body::to_bytes(body).await {
        Ok(bytes) => bytes,
        Err(err) => {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("failed to read {} body: {}", direction, err),
            ));
        }
    };

    if let Ok(body) = std::str::from_utf8(&bytes) {
        println!("{} body = {}", direction, body);
    }

    Ok(bytes)
}