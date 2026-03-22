# dplane/src/core/thread.rs

## 概要
スレッド管理の抽象化層。

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
- `thread.rs`: スレッドスポーン

## DPDK vs Linux

| 機能 | DPDK | Linux |
|------|------|-------|
| スレッド | lcore | std::thread |
| 初期化 | thread_init() | 不要 |
| スポーン | rte_eal_remote_launch | thread::spawn |
