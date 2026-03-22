# dplane/src/core/runtime.rs

## 概要
WASMランタイムの抽象化層。

## モジュール構造
```rust
#[cfg(feature="runtime_wasm")]
pub mod wasm;
```

## サブモジュール
- `wasm/wasmer/runtime.rs`: Wasmerランタイム
