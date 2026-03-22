# dplane/src/worker/pipeline.rs

## 概要
パイプラインワーカー。WASMパイプラインを実行し、パケットを処理。

## 構造体

### PipelineArgs
ワーカー引数。

```rust
pub struct PipelineArgs {
    pub id: usize,                    // ワーカーID
    pub pipeline: Pipeline,           // WASMパイプライン
    pub ring_from_rx: Ring,          // RX/Cacheからの入力リング
    pub batch_count: usize,          // バッチサイズ
    pub tx_ring_list: Array<Array<Ring>>, // TXリング (port -> queue)
    pub cache_creater_ring: Ring,    // キャッシュ作成ワーカーへ
}
```

## 関数

### start_pipeline(args_ptr: *mut c_void) -> i32
パイプラインワーカーメインループ。

**処理フロー:**
1. 入力リングからパケットをデキュー
2. 各パケットに対して:
   - キャッシュヒット → `run_cache_pipeline()`
   - キャッシュミス → `run_pipeline()`
3. 出力処理:
   - Port: 対応するTXリングへ
   - Controller: CPUポートへ
   - Drop: パケット解放
4. キャッシュミスの場合、キャッシュ作成ワーカーへ通知

### output_pkt(tx_ring_list, next_tx_queue_list, pktbuf, output) -> bool
パケットを出力。

**戻り値:**
- `true`: 送信成功
- `false`: ドロップ

## キャッシュパイプライン vs フルパイプライン
```rust
if is_cache_hit {
    // テーブルルックアップスキップ
    pipeline.run_cache_pipeline(pkt, len, &parse_result, &cache_data, &mut output);
} else {
    // フルパイプライン実行
    pipeline.run_pipeline(pkt, len, &parse_result, &mut cache_data, &mut output);
    // キャッシュ作成通知
    cache_creater_ring.enqueue(pkt_analysis_result);
}
```

## 関連ファイル
- `rx.rs`: RXワーカー
- `cache.rs`: キャッシュワーカー
- `tx.rs`: TXワーカー
- `cache_creater.rs`: キャッシュ作成ワーカー
- `pipeline/pipeline.rs`: パイプライン実装
