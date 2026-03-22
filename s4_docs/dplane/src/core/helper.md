# dplane/src/core/helper.rs

## 概要
プラットフォーム初期化ヘルパーのモジュール宣言。

## モジュール構造
```rust
#[cfg(feature="dpdk")]
pub mod dpdk;

#[cfg(feature="linux")]
pub mod linux;
```

## サブモジュール
- `dpdk.rs`: DPDK EAL初期化
- `linux.rs`: Linuxヒープ初期化

## 使用方法
```rust
// DPDKモード
#[cfg(feature="dpdk")]
let args_start = core::helper::dpdk::init();

// Linuxモード
#[cfg(feature="linux")]
core::helper::linux::init();
```
