# dplane/src/core/network/dpdk/interface.rs

## 概要
DPDKを使用したネットワークインターフェースの管理。ポートの初期化、RX/TX操作を担当。

## 構造体

### Interface
```rust
pub struct Interface {
    pub port: u16,   // DPDKポートID
    pub queue: u16,  // キューID
}
```

## 関数

### Interface::init(name: &str, rx_queues: u16, tx_queues: u16) -> (u16, u16, u16)
インターフェースを初期化。

**引数:**
- `name`: デバイス名 (例: "0000:06:00.0", "net_tap0")
- `rx_queues`: 希望RXキュー数
- `tx_queues`: 希望TXキュー数

**戻り値:**
- `(port_id, actual_rx_queues, actual_tx_queues)`

**処理:**
1. デバイス名からポートIDを検索
2. デバイス情報から最大キュー数を取得
3. キュー数を調整
4. ポートを起動

### Interface::find_interface_from_name(name: &str) -> u16
デバイス名からポートIDを検索。

### Interface::up(port: u16, rx_queues: u16, tx_queues: u16)
ポートを起動。

**設定内容:**
- RXキューセットアップ (1024バッファ、262144要素のmempool)
- TXキューセットアップ (1024バッファ)
- プロミスキャスモード有効化
- リンク状態確認

### interface.rx(&self, pktbuf: &Array<PktBuf>, len: usize) -> u16
パケットを受信。

**戻り値:**
- 受信パケット数

### interface.tx(&self, pktbuf: &Array<&mut RawPktBuf>, len: usize) -> u16
パケットを送信。

**戻り値:**
- 送信成功パケット数

### interface.debug_show_info(&self)
デバッグ情報を表示。

## 設定パラメータ
```rust
// RXキュー
mempool elements: 262144
mempool cache: 512
rx_queue_size: 1024

// TXキュー
tx_queue_size: 1024
```

## 使用例
```rust
let (port, rx_q, tx_q) = Interface::init("0000:06:00.0", 4, 4);

let iface = Interface { port, queue: 0 };
let pktbuf = Array::<PktBuf>::new(64);
let count = iface.rx(&pktbuf, 64);
```

## 出力情報
初期化時に以下を表示:
- デバイス名
- 最大RX/TXキュー数
- MTU範囲
- リンク状態
- リンク速度
- MACアドレス

## 関連ファイル
- `pktbuf.rs`: パケットバッファ
- `worker/rx.rs`: RXワーカー
- `worker/tx.rs`: TXワーカー
