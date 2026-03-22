# dplane/src/deparser.rs

## 概要
パケット再構築モジュール。変更されたヘッダーをパケットに書き戻す。

## サブモジュール
```rust
pub mod deparser;           // デパーサー本体
pub mod runtime_native_api; // ネイティブAPI
```

## 役割
1. パイプラインで変更されたヘッダーフィールドをパケットに反映
2. ヘッダーの追加/削除対応 (将来)
3. チェックサム再計算 (将来)

## 現状
モジュール定義のみで、実装は最小限。パイプラインWASM内で直接パケット変更を行っている。

## 関連ファイル
- `deparser/deparser.rs`: デパーサー実装
- `deparser/runtime_native_api.rs`: ネイティブAPI
