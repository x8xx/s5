# dplane/src/worker/cache.rs

## 概要
キャッシュワーカー。L2/L3キャッシュをチェックし、パイプラインへ転送。

## 構造体

### CacheArgs
ワーカー引数。

```rust
pub struct CacheArgs<'a> {
    pub id: usize,                           // ワーカーID
    pub ring: Ring,                          // RXからの入力リング
    pub batch_count: usize,                  // バッチサイズ
    pub buf_size: usize,                     // バッファサイズ
    pub header_max_size: usize,              // 最大ヘッダーサイズ
    pub l2_cache: Array<RwLock<CacheElement>>, // L2キャッシュ
    pub l3_cache: &'a TupleSpace<'a>,        // L3 TSS
    pub pipeline_ring_list: Array<Ring>,    // パイプラインへのリング
}
```

## 関数

### start_cache(cache_args_ptr: *mut c_void) -> i32
キャッシュワーカーメインループ。

**処理フロー:**
1. RXからパケットをデキュー
2. LBFヒットの場合:
   - L2キャッシュをチェック
   - ヒット → キャッシュデータをコピー、パイプラインへ
3. L3キャッシュをチェック (現在コメントアウト)
4. パイプラインワーカーへ転送

### next_core(current_core, core_limit) -> usize
次のパイプラインコアを選択 (ラウンドロビン)。

## L2キャッシュ処理
```rust
if is_lbf_hit {
    let cache = l2_cache[l2_hash].read();
    if cache.cmp_ptr_key(l2_key, l2_key_len) {
        // ヒット
        cache_data = cache.data.clone();
        is_cache_hit = true;
    }
}
```

## 関連ファイル
- `rx.rs`: 入力元
- `pipeline.rs`: 出力先
- `cache/tss.rs`: L3キャッシュ
