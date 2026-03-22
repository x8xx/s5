# dplane/src/core.rs

## 概要
プラットフォーム抽象化層のルートモジュール。DPDK/Linuxの両方をサポートするための共通インターフェースを提供。

## サブモジュール
```rust
pub mod helper;   // 環境初期化ヘルパー
pub mod logger;   // ロギング
pub mod memory;   // メモリ管理
pub mod network;  // ネットワークI/O
pub mod thread;   // スレッド管理
pub mod runtime;  // WASMランタイム
```

## アーキテクチャ設計
各サブモジュールは以下のパターンで実装:

```
core/xxx.rs           # モジュール宣言
core/xxx/dpdk.rs      # DPDK実装
core/xxx/linux.rs     # Linux実装
```

フィーチャーフラグによる条件付きコンパイル:
```rust
#[cfg(feature="dpdk")]
pub mod dpdk;
#[cfg(feature="linux")]
pub mod linux;
```

## モジュール詳細

### helper/
- `dpdk.rs`: DPDK EAL初期化、Hugepages管理
- `linux.rs`: Linuxヒープ初期化

### logger/
- `log.rs`: ログマクロ定義
- `lib/std/write.rs`: 標準出力ライター

### memory/
- `heap.rs`: ヒープアロケーター (DPDK memzone / malloc)
- `array.rs`: 固定長配列
- `ring.rs`: ロックフリーリングバッファ
- `ptr.rs`: スマートポインタ
- `vector.rs`: 可変長配列
- `linear_list.rs`: リニアリスト

### network/
- `interface.rs`: ネットワークインターフェース
- `pktbuf.rs`: パケットバッファ (mbuf)

### thread/
- `thread.rs`: スレッドスポーン (lcore / std::thread)

### runtime/
- `wasm/wasmer/runtime.rs`: Wasmer WASM実行環境

## DPDK vs Linux実装の違い

| 機能 | DPDK | Linux |
|------|------|-------|
| メモリ | rte_memzone | malloc |
| リング | rte_ring | VecDeque |
| パケット | rte_mbuf | Vec<u8> |
| スレッド | rte_eal_remote_launch | std::thread |
| NIC | rte_eth_* | pnet |

## 関連ファイル
- 各サブモジュールの詳細はそれぞれのドキュメントを参照
