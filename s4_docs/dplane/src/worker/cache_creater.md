# dplane/src/worker/cache_creater.rs

## 概要
キャッシュ作成ワーカー。パイプライン処理結果をキャッシュに登録。

## 構造体

### CacheCreaterArgs
ワーカー引数。

```rust
pub struct CacheCreaterArgs {
    pub ring: Ring,                                  // パイプラインからの入力
    pub header_list: Array<Header>,                  // ヘッダー定義
    pub table_list: Array<RwLock<Table>>,           // テーブルリスト
    pub l1_cache_list: Array<Array<RwLock<CacheElement>>>, // L1キャッシュ
    pub lbf_list: Array<Array<RwLock<u64>>>,        // LBF
    pub l2_cache_list: Array<Array<Array<RwLock<CacheElement>>>>, // L2キャッシュ
}
```

## 関数

### start_cache_creater(args_ptr: *mut c_void) -> i32
キャッシュ作成ワーカーメインループ。

**処理フロー:**
1. リングからPktAnalysisResultをデキュー
2. 各結果に対して:
   - L1キャッシュを更新
   - L2キャッシュを更新
   - LBFを更新
   - (L3キャッシュ更新 - コメントアウト)
3. PktAnalysisResultを解放

## キャッシュ更新処理

### L1キャッシュ
```rust
let mut l1_cache = l1_cache_list[rx_id][l1_hash].write();
l1_cache.key_len = l1_key_len;
l1_key.deepcopy(&mut l1_cache.key);
cache_data.deepcopy(&mut l1_cache.data);
```

### L2キャッシュ
```rust
let mut l2_cache = l2_cache_list[rx_id][cache_id][l2_hash].write();
l2_cache.key_len = l2_key_len;
l2_key.deepcopy(&mut l2_cache.key);
cache_data.deepcopy(&mut l2_cache.data);
```

### LBF更新
```rust
let mut core_flag = lbf_list[rx_id][l2_hash].write();
*core_flag |= 1 << cache_id;  // キャッシュコアのビットを立てる
```

## L3キャッシュ (未実装)
コメントアウトされたコードにTSS更新ロジックの骨格あり:
- フローエントリからタプルフィールドを抽出
- タプルハッシュを計算
- TupleSpaceに登録

## 関連ファイル
- `pipeline.rs`: 入力元
- `cache/cache.rs`: キャッシュ構造
- `cache/tss.rs`: L3 TSS
