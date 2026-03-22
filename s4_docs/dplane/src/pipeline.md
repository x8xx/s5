# dplane/src/pipeline.rs

## 概要
パケット処理パイプラインモジュール。WASMで実装されたパイプラインを実行し、テーブルルックアップとアクション適用を行う。

## サブモジュール
```rust
pub mod pipeline;           // パイプライン本体
pub mod table;              // フローテーブル
pub mod tree;               // ツリー構造 (AVL, Radix)
pub mod runtime_native_api; // ネイティブAPI
pub mod tx_conf;            // 送信設定
pub mod output;             // 出力先定義
```

## 役割
1. テーブルルックアップ
2. アクション選択・実行
3. パケット変更
4. 出力先決定
5. キャッシュデータ生成

## 関連ファイル
- `pipeline/pipeline.rs`: パイプライン実装
- `pipeline/table.rs`: テーブル実装
- `pipeline/tree/`: ツリー構造
