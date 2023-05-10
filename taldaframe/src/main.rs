use std::{sync::{Arc, RwLock}, collections::HashMap, borrow::Cow};

use axum::{
    routing::get,
    Router, response::{IntoResponse, Response}, extract::{Path, State, DefaultBodyLimit, Query, self}, body::{Bytes, Body}, http::{StatusCode, Request}, BoxError, handler::Handler, middleware::{Next, self}, error_handling::HandleErrorLayer,
};
use base64::{Engine as Base64Engine, engine::general_purpose};
use serde::Deserialize;
use tower::ServiceBuilder;
use tower_http::{compression::CompressionLayer, limit::{RequestBodyLimitLayer}};
use wasmtime::{Engine, Module, Store, Instance, Linker};

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
) -> impl IntoResponse {
    let wasm_functions = &state.read().unwrap().wasm_functions;

    if let Some(value) = wasm_functions.get(&key) {
        let engine = Engine::default();
        let module = Module::new(&engine, &value.wasm_bytecode);
        if let Err(e) = module {
            println!("wasm_app_execute: {:?}", e);
            return Err(StatusCode::BAD_REQUEST);
        }
        let module = module.unwrap();

        let mut linker: Linker<()> = Linker::new(&engine);
        if let Err(e) = linker.func_wrap("http", "body", || 0) {
            println!("wasm_app_execute: {:?}", e);
            return Err(StatusCode::BAD_REQUEST);
        }
        if let Err(e) = linker.func_wrap("http", "body_len", || 0) {
            println!("wasm_app_execute: {:?}", e);
            return Err(StatusCode::BAD_REQUEST);
        }
        if let Err(e) = linker.func_wrap("http", "method", || 0) {
            println!("wasm_app_execute: {:?}", e);
            return Err(StatusCode::BAD_REQUEST);
        }
        if let Err(e) = linker.func_wrap("http", "method_len", || 0) {
            println!("wasm_app_execute: {:?}", e);
            return Err(StatusCode::BAD_REQUEST);
        }
        if let Err(e) = linker.func_wrap("http", "path", || 0) {
            println!("wasm_app_execute: {:?}", e);
            return Err(StatusCode::BAD_REQUEST);
        }
        if let Err(e) = linker.func_wrap("http", "path_len", || 0) {
            println!("wasm_app_execute: {:?}", e);
            return Err(StatusCode::BAD_REQUEST);
        }


        let mut store = Store::new(&engine, ());
        let instance = linker.instantiate(&mut store, &module);
        if let Err(e) = instance {
            println!("wasm_app_execute: {:?}", e);
            return Err(StatusCode::BAD_REQUEST);
        }
        let instance = instance.unwrap();

        let memory = instance
            .get_memory(&mut store, "memory")
            .ok_or(StatusCode::BAD_REQUEST)?;
        let size = instance.get_typed_func::<(), u32>(&mut store, "size").or(Err(StatusCode::BAD_REQUEST))?;
        let load_fn = instance.get_typed_func::<u32, u32>(&mut store, "load").or(Err(StatusCode::BAD_REQUEST))?;
        let store_fn = instance.get_typed_func::<(u32, u32), ()>(&mut store, "store").or(Err(StatusCode::BAD_REQUEST))?;

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
        get(exec_wasm_function)
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