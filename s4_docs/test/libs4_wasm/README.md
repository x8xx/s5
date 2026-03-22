# libs4_wasm - WASM Library Stubs

## 概要
Parser/Pipeline WASMモジュール用のライブラリスタブ。ネイティブAPIの外部関数宣言を提供。

## ファイル

### libparser.rs
パーサー用ネイティブAPI。

```rust
extern "C" {
    pub fn s4_sys_pkt_get_len(args_ptr: i64) -> usize;
    pub fn s4_sys_pkt_read(args_ptr: i64, offset: i32) -> i32;
    pub fn s4_sys_pkt_drop(args_ptr: i64);
    pub fn s4_sys_extract_hdr(args_ptr: i64, hdr_id: i32, offset: i32, size: i32);
}
```

### libpipeline.rs
パイプライン用ネイティブAPI。

```rust
extern "C" {
    pub fn s4_sys_debug(args_ptr: i64);
    pub fn s4_sys_table_search(args_ptr: i64, table_id: i32) -> i64;
    pub fn s4_sys_pkt_get_header_len(args_ptr: i64) -> i32;
    pub fn s4_sys_pkt_get_payload_len(args_ptr: i64) -> i32;
    pub fn s4_sys_pkt_read(args_ptr: i64, offset: i32) -> i32;
    pub fn s4_sys_pkt_write(args_ptr: i64, offset: i32, value: i32);
    pub fn s4_sys_metadata_read(args_ptr: i64, index: i32) -> i32;
    pub fn s4_sys_action_get_id(action_set_ptr: i64) -> i32;
    pub fn s4_sys_action_get_data(action_set_ptr: i64, index: i32) -> i32;
    pub fn s4_sys_output_port(args_ptr: i64, port: i32);
    pub fn s4_sys_output_all(args_ptr: i64);
    pub fn s4_sys_output_controller(args_ptr: i64);
    pub fn s4_sys_output_drop(args_ptr: i64);
}
```

## 使用方法
WASMモジュール内でinclude:

```rust
mod libparser;
use libparser::*;

// または
mod libpipeline;
use libpipeline::*;
```

## 関連ファイル
- `dplane/src/parser/runtime_native_api.rs`
- `dplane/src/pipeline/runtime_native_api.rs`
