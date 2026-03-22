# dplane/src/core/network/dpdk/pktbuf.rs

## 概要
DPDKのrte_mbufをラップしたパケットバッファ。パケットデータへのアクセスとライフサイクル管理を提供。

## 型定義

### RawPktBuf
```rust
pub type RawPktBuf = dpdk_sys::rte_mbuf;
```
生のDPDK mbuf型。TX時に使用。

## 構造体

### PktBuf
```rust
pub struct PktBuf {
    pub buf: *mut rte_mbuf,
}
```

## 関数

### pktbuf.get_raw_pkt(&self) -> (*mut u8, usize)
パケットデータへのポインタと長さを取得。

**戻り値:**
- `*mut u8`: パケットデータ先頭ポインタ
- `usize`: パケット長

**処理:**
1. nullチェック
2. `buf_addr` + `data_off` でデータ先頭を計算
3. `data_len` でパケット長を取得

### pktbuf.as_raw(&mut self) -> &mut RawPktBuf
生のrte_mbuf参照を取得。TX時に使用。

### pktbuf.free(&self)
パケットバッファを解放。

```rust
rte_pktmbuf_free(self.buf);
```

## rte_mbufの構造
```
+------------------+
| rte_mbuf header  |
+------------------+
| headroom         | <- data_off
+------------------+
| packet data      | <- buf_addr + data_off
| (data_len bytes) |
+------------------+
| tailroom         |
+------------------+
```

## 使用例
```rust
// RXから取得
let (pkt_ptr, pkt_len) = pktbuf.get_raw_pkt();
if pkt_len == 0 {
    continue;  // 無効なパケット
}

// パケット処理
process_packet(pkt_ptr, pkt_len);

// TXへ送信
tx_ring.enqueue(pktbuf.as_raw());

// または解放
pktbuf.free();
```

## デバッグログ
```rust
debug_log!("Pktbuf Free {:x}", self.buf as i64);
```

## 注意点
- `get_raw_pkt()`でnullの場合は`(null_mut(), 0)`を返す
- `free()`は所有権を消費しない (明示的な解放が必要)
- マルチバッファ (scatter-gather) は未対応

## 関連ファイル
- `interface.rs`: RX/TX操作
- `worker/rx.rs`: パケット受信
- `worker/tx.rs`: パケット送信
