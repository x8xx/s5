# dplane/src/core/memory/dpdk/heap.rs

## 概要
DPDKのmemzoneを使用したヒープアロケーター。Hugepages上にメモリを確保し、高速なメモリアクセスを実現。

## グローバル変数

### HEAP
```rust
pub static mut HEAP: RwLock<Heap>
```
グローバルヒープインスタンス。全スレッドから共有。

## 構造体

### Heap
```rust
pub struct Heap {
    memzones: *mut *const rte_memzone,  // memzoneポインタ配列
    current_memzone: isize,              // 現在のmemzoneインデックス
    max_memzone_num: isize,              // 最大memzone数
    memzone_data_size: isize,            // 各memzoneのサイズ
    next_pos: isize,                     // 次の割り当て位置
}
```

## 関数

### Heap::new() -> &'static RwLock<Self>
グローバルHEAPへの参照を取得。

### Heap::init(memzone_data_size: usize, max_memzone_num: usize)
ヒープを初期化。

**処理:**
1. memzoneポインタ配列用のmemzoneを確保
2. データ用memzoneを`max_memzone_num`個確保
3. グローバルHEAPを更新

**パラメータ:**
- `memzone_data_size`: 各memzoneのサイズ (デフォルト: 1GB)
- `max_memzone_num`: memzone数 (デフォルト: 15)

### heap.malloc<T>(&mut self, size: usize) -> *mut T
メモリを確保。

**処理:**
1. 必要サイズを計算 (`size_of::<T>() * size`)
2. 現在のmemzoneに空きがあるかチェック
3. 不足なら次のmemzoneに切り替え
4. ポインタを返却、`next_pos`を更新

**制限:**
- 単一確保が`memzone_data_size`を超えるとpanic
- フリーは未実装 (リニアアロケーター)

## メモリレイアウト
```
memzones[0]: [ data area (1GB) ] <- current
memzones[1]: [ data area (1GB) ]
...
memzones[14]: [ data area (1GB) ]
```

## 使用例
```rust
// 初期化 (通常はhelper::dpdk::init()で呼ばれる)
Heap::init(1073741824, 15);

// メモリ確保
let mut heap = Heap::new().write().unwrap();
let ptr: *mut u8 = heap.malloc::<u8>(1024);
```

## 注意点
- リニアアロケーターのため、フリーできない
- メモリが枯渇するとpanic
- スレッドセーフ (RwLockで保護)

## 関連ファイル
- `array.rs`: 配列型 (Heapを使用)
- `ring.rs`: リングバッファ (独自memzone)
- `helper/dpdk.rs`: 初期化呼び出し
