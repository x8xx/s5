# test/testdata/l2switch/parser/parser.rs

## 概要
L2スイッチ用のパーサーWASMモジュール。Ethernetヘッダーを抽出。

## 属性
```rust
#![no_main]  // スタンドアロンWASMとしてコンパイル
```

## 依存
```rust
mod libparser;
use libparser::*;  // ネイティブAPI
```

## 関数

### parse(parser_args_ptr: i64) -> bool
WASMエクスポート関数。

**処理:**
1. `s4_sys_get_pkt_len()` でパケット長取得
2. `parse_ethernet()` 呼び出し

### parse_ethernet(parser_args_ptr: i64, pkt_len: usize) -> bool
Ethernetヘッダーをパース。

**処理:**
1. パケット長チェック (>14バイト)
2. `s4_sys_extract_hdr()` でヘッダー抽出
   - hdr_id: 0
   - offset: 0
   - size: 14

**戻り値:**
- `true`: パース成功
- `false`: パケット短すぎ

## ネイティブAPI

### s4_sys_get_pkt_len(args_ptr) -> usize
パケット長を取得。

### s4_sys_extract_hdr(args_ptr, hdr_id, offset, size)
ヘッダーを抽出。parse_resultに登録。

## コンパイル
```bash
rustc --target wasm32-unknown-unknown -O --crate-type=cdylib parser.rs -o parser.wasm
```

## 関連ファイル
- `libparser.rs`: ネイティブAPIスタブ
- `dplane/src/parser/runtime_native_api.rs`: ネイティブAPI実装
