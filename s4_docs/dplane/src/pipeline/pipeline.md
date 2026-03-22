# dplane/src/pipeline/pipeline.rs

## 概要
WASMベースのパケット処理パイプライン。テーブルルックアップとアクション適用を実行。

## 構造体

### Pipeline
```rust
pub struct Pipeline {
    runtime: Runtime,                    // WASMランタイム
    runtime_args: RuntimeArgs,           // WASM関数引数
    table_list: Array<RwLock<Table>>,   // フローテーブルリスト
}
```

## コンストラクタ

### Pipeline::new(wasm: &[u8], table_list: Array<RwLock<Table>>) -> Self
WASMバイトコードからパイプラインを作成。

**登録されるネイティブ関数:**
- `s4_sys_debug`: デバッグ出力
- `s4_sys_table_search`: テーブル検索
- `s4_sys_pkt_get_header_len`: ヘッダー長取得
- `s4_sys_pkt_get_payload_len`: ペイロード長取得
- `s4_sys_pkt_read`: パケット読み取り
- `s4_sys_pkt_write`: パケット書き込み
- `s4_sys_metadata_read`: メタデータ読み取り
- `s4_sys_action_get_id`: アクションID取得
- `s4_sys_action_get_data`: アクションデータ取得
- `s4_sys_output_port`: 出力ポート指定
- `s4_sys_output_all`: 全ポート出力
- `s4_sys_output_controller`: コントローラ送信
- `s4_sys_output_drop`: ドロップ

## メソッド

### run_pipeline(&mut self, pkt, pkt_len, parse_result, cache_data, output)
パイプラインを実行 (キャッシュミス時)。

**処理:**
1. `RuntimeArgs`を構築 (`is_cache: false`)
2. WASM関数`run_pipeline`を呼び出し
3. `cache_data`にテーブルルックアップ結果を格納
4. `output`に出力先を設定

### run_cache_pipeline(&mut self, pkt, pkt_len, parse_result, cache_data, output)
キャッシュヒット時のパイプライン実行。

**違い:**
- `is_cache: true` - テーブルルックアップをスキップ
- `cache_data`から直接アクションを取得

## RuntimeArgs
WASMに渡される引数構造体。

```rust
struct RuntimeArgs {
    table_list: &Array<RwLock<Table>>,
    pkt: *mut u8,
    pkt_len: usize,
    parse_result: &ParseResult,
    is_cache: bool,
    cache_data: &mut CacheData,
    output: &mut Output,
}
```

## 使用例
```rust
let pipeline = Pipeline::new(&pipeline_wasm, table_list);

// キャッシュミス時
let mut cache_data = CacheData::new(table_count);
let mut output = Output::Drop;
pipeline.run_pipeline(pkt, len, &parse_result, &mut cache_data, &mut output);

// キャッシュヒット時
pipeline.run_cache_pipeline(pkt, len, &parse_result, &mut cache_data, &mut output);
```

## 関連ファイル
- `runtime_native_api.rs`: ネイティブAPI実装
- `table.rs`: テーブル実装
- `output.rs`: 出力先定義
- `worker/pipeline.rs`: 呼び出し元
