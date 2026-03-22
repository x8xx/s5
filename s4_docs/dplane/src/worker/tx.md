# dplane/src/worker/tx.rs

## 概要
パケット送信ワーカー。パイプラインからパケットを受け取り、NICへ送信。

## 構造体

### TxArgs
ワーカー引数。

```rust
pub struct TxArgs {
    pub id: usize,           // ワーカーID
    pub interface: Interface, // ネットワークインターフェース
    pub ring: Ring,          // パイプラインからの入力リング
    pub batch_count: usize,  // バッチサイズ
}
```

## 関数

### start_tx(tx_args_ptr: *mut c_void) -> i32
TXワーカーメインループ。

**処理フロー:**
1. リングからパケットをバーストデキュー
2. `interface.tx()`でバースト送信
3. 繰り返し

## 実装詳細
```rust
loop {
    let count = ring.dequeue_burst::<RawPktBuf>(&raw_pktbuf_list, batch_count);
    if count > 0 {
        let success = interface.tx(&raw_pktbuf_list, count);
        // 送信失敗分はドロップ (現在の実装)
    }
}
```

## パフォーマンス考慮
- バースト送信で効率化
- ポーリングモードでレイテンシ最小化

## 関連ファイル
- `pipeline.rs`: 入力元
- `core/network/dpdk/interface.rs`: NIC送信
