# dplane/src/core/network.rs

## 概要
ネットワークI/Oの抽象化層。

## モジュール構造
```rust
#[cfg(feature="dpdk")]
pub mod dpdk;
#[cfg(feature="dpdk")]
pub use self::dpdk::*;

#[cfg(feature="linux")]
pub mod linux;
#[cfg(feature="linux")]
pub use self::linux::*;
```

## サブモジュール
- `interface.rs`: ネットワークインターフェース
- `pktbuf.rs`: パケットバッファ

## DPDK vs Linux

| 機能 | DPDK | Linux |
|------|------|-------|
| インターフェース | rte_eth_* | pnet |
| パケットバッファ | rte_mbuf | Vec<u8> |
| RX/TX | poll mode | システムコール |
| ゼロコピー | Yes | No |

## 関連ファイル
- `dpdk/interface.rs`
- `dpdk/pktbuf.rs`
- `linux/interface.rs`
- `linux/pktbuf.rs`
