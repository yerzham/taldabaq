# taldabaq

## Pre-requisites

Install `wit-deps-cli` to manage WIT dependencies

```
cargo install wit-deps-cli
```

Install `wasm-tools`

```
cargo install wasm-tools
```

Install JavaScript Wasm Component Tools
```
npm install -g @bytecodealliance/jco
```

Use rust nightly toolchain

## Build

Rust Host Demo

```
cargo +nightly watch -q -c -w src/ -x run
cargo +nightly watch -q -c -w tests/ -x "test -- --nocapture"
```

JavaScript Host Demo

```
jco transpile http_endpoint_{name}_component.wasm -o wasm --map taldawasm:main/http-outgoing=../target/fetch.js
npm run start
```

Build a wasm component

```
cargo build --target wasm32-unknown-unknown --release
wasm-tools component new ./target/wasm32-unknown-unknown/release/http_endpoint_example.wasm -o ./target/wasm32-unknown-unknown/release/http_endpoint_component.wasm
```