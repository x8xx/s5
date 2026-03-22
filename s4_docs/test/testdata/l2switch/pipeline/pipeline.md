# test/testdata/l2switch/pipeline/pipeline.rs

## 概要
L2スイッチ用のパイプラインWASMモジュール。テーブルルックアップとアクション実行。

## 属性
```rust
#![no_main]  // スタンドアロンWASMとしてコンパイル
```

## 依存
```rust
mod libpipeline;
use libpipeline::*;  // ネイティブAPI
```

## 関数

### run_pipeline(pipeline_args: i64)
WASMエクスポート関数。

**処理:**
1. `s4_sys_search_table()` でテーブル0を検索
2. `s4_sys_get_action_id()` でアクションID取得
3. アクションに応じて処理:

| action_id | 処理 |
|-----------|------|
| 0 | ポート出力: `s4_sys_set_metadata()` |
| 1 | フラッディング: `s4_sys_flooding()` |
| _ | ドロップ: `s4_sys_drop()` |

## ネイティブAPI

### s4_sys_search_table(args_ptr, table_id) -> action_set_ptr
テーブルを検索し、アクションセットを取得。

### s4_sys_get_action_id(action_set_ptr) -> action_id
アクションIDを取得。

### s4_sys_get_action_data(action_set_ptr, index) -> data
アクションデータを取得。

### s4_sys_set_metadata(args_ptr, index, value)
メタデータを設定。

### s4_sys_flooding(args_ptr)
全ポートに出力。

### s4_sys_drop(args_ptr)
パケットをドロップ。

## L2スイッチロジック
```
dst_mac → テーブル検索 → action
  ↓
既知MAC → 特定ポート出力 (action_id=0)
未知MAC → フラッディング (action_id=1)
デフォルト → ドロップ (action_id=2)
```

## コンパイル
```bash
rustc --target wasm32-unknown-unknown -O --crate-type=cdylib pipeline.rs -o pipeline.wasm
```

## 関連ファイル
- `libpipeline.rs`: ネイティブAPIスタブ
- `dplane/src/pipeline/runtime_native_api.rs`: ネイティブAPI実装
