# taldabaq
Deploy and manage IoT applications

```
cargo +nightly watch -q -c -w src/ -x run
cargo +nightly watch -q -c -w tests/ -x "test -q http_integration_test -- --nocapture"

cargo build --release --target=wasm32-unknown-unknown
```