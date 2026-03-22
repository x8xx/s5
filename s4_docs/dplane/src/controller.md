# dplane/src/controller.rs

## 概要
メインコントローラモジュール。ワーカーの起動とTCPサーバーを管理。

## サブモジュール
```rust
mod cmd;              // コマンド定義
mod cp_stream;        // CP通信ハンドラ
mod table_controller; // テーブル操作
mod to_cpu;           // CPU送信 (未使用)
```

## 関数

### start_controller(switch_config: &SwitchConfig)
メインコントローラを起動。

**処理フロー:**

1. **データベース初期化**
   - ヘッダーリスト作成
   - テーブルリスト作成
   - 初期フローエントリ登録

2. **キャッシュ初期化**
   - L1キャッシュ (インターフェースごと)
   - L2キャッシュ (インターフェース×キャッシュコア)
   - LBF
   - L3 TupleSpace

3. **リング初期化**
   - RX→Cache
   - RX/Cache→Pipeline
   - Pipeline→TX
   - Pipeline→CacheCreator

4. **ワーカー起動**
   - Pipeline Workers
   - Cache Workers
   - TX Workers
   - RX Workers
   - Cache Creator

5. **TCPサーバー起動**
   - コントロールプレーン接続受付
   - `cp_stream::stream_handler`で処理

## メモリサイズ設定
```rust
let pktbuf_size = 16777216;       // 16M
let cache_ring_size = 65536;      // 64K
let pipeline_ring_size = 65536;   // 64K
let tx_ring_size = 65536;         // 64K
```

## バッチサイズ設定
```rust
let rx_batch_count = 8192;
let cache_batch_count = 8192;
let pipeline_batch_count = 8192;
let tx_batch_count = 16;
```

## 関連ファイル
- `cmd.rs`: コマンド定義
- `cp_stream.rs`: CP通信ハンドラ
- `table_controller.rs`: テーブル操作
- `worker/`: 各ワーカー
