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
                       [Cache Creater]
```

## 処理フロー

### 1. RX Worker (`worker/rx.rs`)
- パケット受信 (NICからバッチ受信)
- パーサー実行 (WASM)
- L1キャッシュチェック (ヘッダー全体ハッシュ)
  - HIT: Pipeline Workerへ直接転送
  - MISS: L2キー生成 → LBFでCache Worker選択 → Cache Workerへ転送

### 2. Cache Worker (`worker/cache.rs`)
- LBFヒット時のみL2キャッシュチェック
  - HIT: Pipeline Workerへ転送 (キャッシュパイプライン)
  - MISS: Pipeline Workerへ転送 (フルパイプライン)
- LBFミス時: Pipeline Workerへ転送 (フルパイプライン)
- ※ L3 TSS検索は現在無効化されている

### 3. Pipeline Worker (`worker/pipeline.rs`)
- キャッシュヒット: `run_cache_pipeline()` 実行
- キャッシュミス: `run_pipeline()` 実行
  - L1キー (ヘッダーコピー) を保存
  - Cache Createrへキャッシュデータ送信
- 出力先判定 → TX Workerへ転送

### 4. Cache Creater (`worker/cache_creater.rs`)
- L1キャッシュ更新 (RX ID + L1ハッシュ)
- L2キャッシュ更新 (RX ID + Cache ID + L2ハッシュ)
- LBF更新 (Cache IDのビットをセット)
- ※ L3キャッシュ登録は現在無効化されている

### 5. TX Worker (`worker/tx.rs`)
- Ringからパケット取得
- NICへパケット送信

## ワーカー間通信
- RingバッファでMPMC通信
- ラウンドロビンで負荷分散

## 関連ファイル
- 各サブモジュールの詳細参照
