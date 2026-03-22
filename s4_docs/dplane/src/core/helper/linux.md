# dplane/src/core/helper/linux.rs

## 概要
Linux環境での初期化処理。開発・デバッグ用の軽量バックエンド。

## 関数

### init()
Linuxヒープを初期化。

**処理:**
```rust
Heap::init(107374182, 1);  // 約100MB × 1ゾーン
```

## DPDKとの比較
| 項目 | DPDK | Linux |
|------|------|-------|
| ヒープサイズ | 1GB × 15 | 100MB × 1 |
| メモリタイプ | Hugepages | 通常malloc |
| パフォーマンス | 高 | 低 |
| 用途 | 本番 | 開発/テスト |

## 使用例
```rust
#[cfg(feature="linux")]
core::helper::linux::init();
```

## 関連ファイル
- `dpdk.rs`: DPDK実装
- `memory/linux/heap.rs`: Linuxメモリ管理
