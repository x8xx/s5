# dplane/src/parser.rs

## 概要
パケットパーサーモジュール。WASMで実装されたパーサーを実行し、パケットヘッダーを解析。

## サブモジュール
```rust
pub mod header;            // ヘッダー/フィールド定義
pub mod parse_result;      // パース結果
pub mod parser;            // パーサー本体
pub mod runtime_native_api; // ネイティブAPI
```

## 役割
1. パケットヘッダーの解析
2. プロトコルスタックの認識
3. フィールド抽出
4. キャッシュキー生成用データ提供

## 関連ファイル
- `parser/parser.rs`: パーサー実装
- `parser/header.rs`: ヘッダー構造定義
- `parser/parse_result.rs`: 結果構造体
- `parser/runtime_native_api.rs`: WASM用API
