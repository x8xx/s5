# dplane/src/worker/rx.rs

## 概要
パケット受信ワーカー。NICからパケットを受信し、パーサーとL1キャッシュを実行。

## 構造体

### RxArgs
ワーカー引数。

```rust
pub struct RxArgs {
    pub id: usize,                              // ワーカーID
    pub interface: Interface,                    // ネットワークインターフェース
    pub parser: Parser,                         // パケットパーサー

    pub batch_count: usize,                     // バッチサイズ
    pub pktbuf_size: usize,                     // パケットバッファ数
    pub table_list_len: usize,                  // テーブル数
    pub header_max_size: usize,                 // 最大ヘッダーサイズ

    pub l1_hash_seed: u32,                      // L1ハッシュシード
    pub l2_hash_seed: u32,                      // L2ハッシュシード
    pub l1_cache: Array<RwLock<CacheElement>>, // L1キャッシュ
    pub lbf: Array<RwLock<u64>>,               // Load Balancer Filter
    pub l2_key_max_len: u8,                     // L2キー最大長

    pub header_list: Array<Header>,             // ヘッダー定義
    pub cache_ring_list: Array<Ring>,          // キャッシュワーカーへのリング
    pub pipeline_ring_list: Array<Ring>,       // パイプラインワーカーへのリング
}
```

### Pkt
パケットコンテナ。

```rust
pub struct Pkt<'a> {
    pub owner_ring: *mut RingBuf<Pkt>,
    pub pktbuf: PktBuf,
    pub raw_pkt: *mut u8,
    pub len: usize,
    pub pkt_analysis_result: &'a mut PktAnalysisResult,
}
```

### PktAnalysisResult
パケット解析結果。

```rust
pub struct PktAnalysisResult<'a> {
    pub owner_ring: *mut RingBuf<PktAnalysisResult>,
    pub rx_id: usize,
    pub cache_id: usize,
    pub parse_result: ParseResult,
    pub cache_data: CacheData,
    pub is_cache_hit: bool,
    pub l1_key: Array<u8>,
    pub l1_key_len: usize,
    pub l1_hash: u16,
    pub l2_key: Array<u8>,
    pub l2_key_len: u8,
    pub l2_hash: u16,
    pub is_lbf_hit: bool,
}
```

## 関数

### start_rx(rx_args_ptr: *mut c_void) -> i32
RXワーカーメインループ。

**処理フロー:**
1. RingBuf初期化 (Pkt, PktAnalysisResult)
2. パケット受信ループ:
   - `interface.rx()`でバースト受信
   - 各パケットに対して:
     1. パーサー実行
     2. L1キャッシュチェック
     3. ヒット → パイプラインへ
     4. ミス → L2キー生成、LBFチェック
     5. キャッシュワーカーへ

### select_cache_core(core_flag, core_len, start_pos) -> usize
LBFからキャッシュコアを選択。

## L1キャッシュ処理
```rust
let l1_hash = l1_hash_function(raw_pkt, hdr_size, seed);
if l1_cache[l1_hash].cmp_ptr_key(raw_pkt, hdr_size) {
    // ヒット → パイプラインへ直接
    pipeline_ring.enqueue(pkt);
}
```

## LBF (Load Balancer Filter)
```rust
let core_flag = lbf[l2_hash];
// core_flag: ビットフラグ (各ビットがキャッシュコアを表す)
// 例: 0b00100 → コア2がこのフローを担当
```

## テスト
- `test_select_cache_core`: コア選択ロジックの検証

## 関連ファイル
- `cache.rs`: キャッシュワーカー
- `pipeline.rs`: パイプラインワーカー
- `parser/parser.rs`: パケットパーサー
