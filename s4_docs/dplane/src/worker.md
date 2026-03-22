# dplane/src/worker.rs

## 概要
パケット処理ワーカーモジュール。並列処理のためのワーカースレッド定義。

## サブモジュール
```rust
pub mod rx;            // 受信ワーカー
pub mod cache;         // キャッシュワーカー
pub mod pipeline;      // パイプラインワーカー
pub mod tx;            // 送信ワーカー
pub mod cache_creater; // キャッシュ作成ワーカー
```

## ワーカー構成
```
[NIC] → [RX] → [Cache] → [Pipeline] → [TX] → [NIC]
                              ↓
                       [Cache Creator]
```

## 処理フロー

### 1. RX Worker
- パケット受信
- パーサー実行
- L1キャッシュチェック
- LBFでキャッシュワーカー選択

### 2. Cache Worker
- L2キャッシュチェック
- L3 TSSチェック
- パイプラインワーカーへ転送

### 3. Pipeline Worker
- キャッシュヒット: キャッシュパイプライン実行
- キャッシュミス: フルパイプライン実行
- キャッシュ作成ワーカーへ通知

### 4. Cache Creator
- L1/L2キャッシュ更新
- LBF更新

### 5. TX Worker
- パケット送信

## ワーカー間通信
- RingバッファでMPMC通信
- ラウンドロビンで負荷分散

## 関連ファイル
- 各サブモジュールの詳細参照
