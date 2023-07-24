# taldabaq

## Requirements

Install `wit-deps-cli` to manage WIT dependencies

```
cargo install wit-deps-cli
```

Install `wasm-tools`

```
cargo install wasm-tools
```

## Build

Deploy and manage IoT applications

```
cargo +nightly watch -q -c -w src/ -x run
cargo +nightly watch -q -c -w tests/ -x "test -q http_integration_test -- --nocapture"

cargo build --release --target=wasm32-unknown-unknown
```

On JavaScript Demo

```
jco transpile http_endpoint_proxy_component.wasm -o wasm --map taldawasm:main/http-outgoing=../target/fetch.js
```

Build a wasm component

```
cargo build --target wasm32-unknown-unknown --release
wasm-tools component new ./target/wasm32-unknown-unknown/release/http_endpoint_example.wasm -o ./target/wasm32-unknown-unknown/release/http_endpoint_component.wasm
```