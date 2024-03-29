
mod tests {
  use anyhow::Result;
  use base64::{engine::general_purpose, Engine as _};
  use serde_json::json;

  #[tokio::test]
  async fn test_http_integration_test() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;
    let wasm_bytecode = include_bytes!("wasm_test_apps/http_endpoint_echo_component.wasm");
    let wasm_bytecode = general_purpose::STANDARD_NO_PAD.encode(wasm_bytecode);

    let response = hc.do_post("/function/echo", json!({
      "wasm_bytecode": wasm_bytecode,
      "options": {
        "wasi": false
      }
    })).await?;

    response.print().await?;
    assert_eq!(response.text_body()?, format!("OK"));

    let response = hc.do_get("/function/echo").await?;
    let response_first_128_letters = response.text_body()?.chars().take(128).collect::<String>();
    println!("echo_first_128_letters: {}", response_first_128_letters);
    assert_eq!(response.text_body()?, wasm_bytecode);

    let response = hc.do_post("/function/echo/execute", &response_first_128_letters).await?;
    response.print().await?;
    assert_eq!(response.text_body()?, format!("Path: /function/echo/execute\nBody:\n{}", response_first_128_letters));

    Ok(())
  }

  #[tokio::test]
  async fn test_http_endpoint_page_upload_test() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;
    let wasm_bytecode = include_bytes!("wasm_test_apps/http_endpoint_page_component.wasm");
    let wasm_bytecode = general_purpose::STANDARD_NO_PAD.encode(wasm_bytecode);

    let response = hc.do_post("/function/hello-world", json!({
      "wasm_bytecode": wasm_bytecode,
      "options": {
        "wasi": false
      }
    })).await?;

    response.print().await?;
    assert_eq!(response.text_body()?, format!("OK"));

    Ok(())
  }

  #[tokio::test]
  async fn test_http_endpoint_proxy_upload_test() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;
    let wasm_bytecode = include_bytes!("wasm_test_apps/http_endpoint_proxy_component.wasm");
    let wasm_bytecode = general_purpose::STANDARD_NO_PAD.encode(wasm_bytecode);

    let response = hc.do_post("/function/proxy", json!({
      "wasm_bytecode": wasm_bytecode,
      "options": {
        "wasi": false
      }
    })).await?;

    response.print().await?;
    assert_eq!(response.text_body()?, format!("OK"));

    Ok(())
  }
}