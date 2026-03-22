# dplane/src/core/helper/dpdk.rs

## 概要
DPDK (Data Plane Development Kit) 環境の初期化とクリーンアップを担当。

## 関数

### gen_random_name() -> CString
UNIXタイムスタンプベースのユニーク名を生成。memzoneやリングの命名に使用。

**戻り値:**
- `CString`: ナノ秒精度のタイムスタンプ文字列

### init() -> i32
DPDKの初期化を実行。

**処理フロー:**
1. コマンドライン引数をCString配列に変換
2. 共有ライブラリをロード (virtio, tap, qede等)
3. `rte_eal_init()` でEALを初期化
4. `thread_init()` でlcore管理を初期化
5. `Heap::init()` でメモリプール初期化 (1GB, 15ゾーン)

**戻り値:**
- スイッチ引数開始インデックス (EAL引数数 + 1)

**ロードされる共有ライブラリ:**
- `rte_virtio_pci_eth_dev`: VirtIO NICドライバ
- `rte_eth_tap`: TAPデバイスドライバ
- `qede_ethdev`: QLogic NICドライバ
- `rte_mempool_ring`: リングバッファmempool

### cleanup()
DPDKの終了処理を実行。

**処理:**
1. `rte_eal_mp_wait_lcore()`: 全lcoreの終了を待機
2. `rte_eal_cleanup()`: EALリソース解放

## メモリ設定
```rust
Heap::init(1073741824, 15);  // 1GB × 15ゾーン = 最大15GBのヒープ
```

## 使用例
```rust
// 初期化
let switch_args_start = init();
let args: Vec<String> = env::args().collect();
let switch_args = &args[switch_args_start..];

// ... アプリケーション処理 ...

// クリーンアップ
cleanup();
```

## 依存関係
- `dpdk_sys`: DPDKのRustバインディング
- `core::thread::thread::thread_init`
- `core::memory::heap::Heap`

## 関連ファイル
- `linux.rs`: Linux実装
- `memory/dpdk/heap.rs`: DPDKメモリ管理
- `thread/dpdk/thread.rs`: lcore管理
