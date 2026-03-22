# dplane/src/cache.rs

## 概要
マルチレベルキャッシュシステム。パケット分類結果をキャッシュし、テーブルルックアップを高速化。

## サブモジュール
```rust
pub mod cache;  // キャッシュ要素
pub mod hash;   // ハッシュ関数
pub mod tss;    // Tuple Space Search (L3)
```

## キャッシュ階層

### L1 Cache
- RXワーカーごとに独立
- キー: パケットヘッダー全体
- ハッシュ: MurmurHash3
- サイズ: 設定可能 (デフォルト65536)

### L2 Cache
- Cacheワーカーごとに独立
- キー: 抽出されたフィールド値
- LBF (Load Balancer Filter) で分散
- サイズ: 設定可能 (デフォルト65536)

### L3 Cache (TSS)
- Tuple Space Search
- フィールド範囲によるタプル分類
- 共有キャッシュ (全ワーカー)

## キャッシュヒット率の最適化
1. L1: ヘッダー完全一致 (高速、低ヒット率)
2. L2: フィールド抽出一致 (中速、中ヒット率)
3. L3: タプルスペース検索 (低速、高ヒット率)

## 関連ファイル
- `cache/cache.rs`: キャッシュ要素
- `cache/hash.rs`: ハッシュ関数
- `cache/tss.rs`: TSS実装
