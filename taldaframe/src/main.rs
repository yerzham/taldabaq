mod http;

use std::{sync::{Arc, RwLock}, collections::HashMap, borrow::Cow};

use axum::{
    routing::{get, any},
    Router, response::{IntoResponse, Response}, extract::{Path, State, DefaultBodyLimit, self}, body::{Bytes, Body}, http::{StatusCode, Request, Method}, BoxError, handler::Handler, middleware::Next, error_handling::HandleErrorLayer,
};
use base64::{Engine as Base64Engine, engine::general_purpose};
use http::taldawasm::http::http_endpoint_types::Request as TWRequest;
use serde::Deserialize;
use tower::ServiceBuilder;
use tower_http::{compression::CompressionLayer, limit::RequestBodyLimitLayer};
use wasmtime::{Engine, Store, Config, component::{Component, Linker}};

type SharedState = Arc<RwLock<AppState>>;

#[derive(Default)]
struct WasmFunction {
    wasm_bytecode: Bytes,
    options: Option<WasmFunctionOptions>,
}

#[derive(Default)]
struct AppState {
    wasm_functions: HashMap<String, WasmFunction>,
}

#[derive(Debug, Deserialize)]
struct WasmFunctionOptions {
    wasi: bool,
}

#[derive(Debug, Deserialize)]
struct SetWasmFunctionParams {
    wasm_bytecode: String,
    options: Option<WasmFunctionOptions>,
}

async fn get_wasm_function(
    Path(key): Path<String>,
    State(state): State<SharedState>,
) -> Result<String, StatusCode> {
    let wasm_functions = &state.read().unwrap().wasm_functions;

    if let Some(value) = wasm_functions.get(&key) {
        Ok(general_purpose::STANDARD_NO_PAD.encode(&value.wasm_bytecode))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

async fn set_wasm_function(Path(key): Path<String>, State(state): State<SharedState>, extract::Json(params): extract::Json<SetWasmFunctionParams>) -> impl IntoResponse {
    let wasm_bytecode = general_purpose::STANDARD_NO_PAD.decode(&mut Bytes::from(params.wasm_bytecode));
    if let Err(e) = wasm_bytecode {
        println!("wasm_app_set: {:?}", e);
        return (StatusCode::BAD_REQUEST, "Invalid base64".to_string());
    }
    let wasm_bytecode = wasm_bytecode.unwrap();
    state.write().unwrap().wasm_functions.insert(key, WasmFunction {
        wasm_bytecode: wasm_bytecode.into(),
        options: params.options,
    });

    (StatusCode::OK, "OK".to_string())
}

async fn exec_wasm_function(
    Path(key): Path<String>,
    State(state): State<SharedState>,
    method: Method,
    body: String,
) -> impl IntoResponse {
    let wasm_functions = &state.read().unwrap().wasm_functions;

    if let Some(value) = wasm_functions.get(&key) {
        let mut config = Config::new();
        config.wasm_component_model(true);
        let engine = Engine::new(&config).map_err(|e| {
            println!("wasm_app_execute: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
        let component = Component::from_binary(&engine, &value.wasm_bytecode).map_err(|e| {
            println!("wasm_app_execute: {:?}", e);
            StatusCode::BAD_REQUEST
        })?;

        let linker: Linker<()> = Linker::new(&engine);

        let mut store = Store::new(&engine, ());
        let (bindings, _instance) = http::Endpoint::instantiate(&mut store, &component, &linker).map_err(|e| {
            println!("wasm_app_execute: {:?}", e);
            StatusCode::BAD_REQUEST
        })?;
        let result = bindings.taldawasm_http_http_endpoint().call_handle_request(&mut store, &TWRequest{
            path: "/".to_string(),
            method: method.into(),
            body: Some(body.as_bytes().to_vec()),
            headers: vec![]
        }).map_err(|e| {
            println!("wasm_app_execute: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        let response = result.map_err(|e| {
            println!("wasm_app_execute: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        let response_str = response.body.map_or(Ok("".to_string()), |v| {
            String::from_utf8(v).map_err(|e| {
                println!("wasm_app_execute: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })
        })?;

        Ok(response_str)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

#[tokio::main]
async fn main() {
    let shared_state = SharedState::default();

    // build our application with a single route
    let app = Router::new().route(
        "/function/:key", 
        get(get_wasm_function.layer(CompressionLayer::new()))
        .post_service(
            set_wasm_function
                .layer((
                    DefaultBodyLimit::disable(), 
                    RequestBodyLimitLayer::new(1024 * 1024 * 10),
                ))
                .with_state(Arc::clone(&shared_state)),
        ),
    ).route(
        "/function/:key/execute",
        any(exec_wasm_function)
    )
    .layer(
        ServiceBuilder::new()
            .layer(HandleErrorLayer::new(handle_error))
            .load_shed()
            .concurrency_limit(1024)
            .timeout(std::time::Duration::from_secs(10))
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