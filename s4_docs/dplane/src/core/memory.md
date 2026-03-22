# dplane/src/core/memory.rs

## 概要
メモリ管理の抽象化層。DPDK/Linuxで異なるメモリアロケーターを提供。

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

### 共通インターフェース
両プラットフォームで同じAPIを提供:

- `heap.rs`: ヒープアロケーター
- `array.rs`: 固定長配列
- `ring.rs`: ロックフリーリングバッファ
- `ptr.rs`: スマートポインタ
- `vector.rs`: 可変長配列
- `linear_list.rs`: リニアリスト

## 主要な型

### Heap
グローバルヒープアロケーター。
```rust
pub static mut HEAP: RwLock<Heap>;
Heap::init(size, zone_count);
Heap::new().write().unwrap().malloc::<T>(count);
```

### Array<T>
固定長配列。Send/Sync実装。
```rust
let arr = Array::<u8>::new(1024);
arr.init(0, value);
arr[0] = value;
```

### Ring
ロックフリーリングバッファ (MPMC)。
```rust
let ring = Ring::new(65536);
ring.enqueue(&mut obj);
ring.dequeue_burst(&buf_array, count);
```

### RingBuf<T>
オブジェクトプール。
```rust
let pool = RingBuf::<Pkt>::new(8192);
let obj = pool.malloc();
pool.free(obj);
```

## DPDK vs Linux

| 型 | DPDK | Linux |
|---|------|-------|
| Heap | rte_memzone | Vec/Box |
| Array | memzone上の配列 | Box<[T]> |
| Ring | rte_ring (MPMC) | Mutex<VecDeque> |

## 関連ファイル
- `dpdk/`: DPDK実装
- `linux/`: Linux実装
