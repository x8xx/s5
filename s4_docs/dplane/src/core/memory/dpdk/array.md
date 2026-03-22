# dplane/src/core/memory/dpdk/array.rs

## 概要
DPDK memzone上に確保される固定長配列。Send/Syncを実装し、スレッド間で安全に共有可能。

## 構造体

### Array<T>
```rust
pub struct Array<T> {
    data: *mut T,  // データポインタ
    len: usize,    // 要素数
}

unsafe impl<T> Send for Array<T> {}
unsafe impl<T> Sync for Array<T> {}
```

## コンストラクタ

### Array::new(len: usize) -> Self
グローバルHeapからメモリを確保。

```rust
let arr = Array::<u8>::new(1024);
```

### Array::new2(len: usize) -> Self
専用memzoneを直接確保 (旧方式)。

### Array::new_manual(data: *mut T, len: usize) -> Self
既存ポインタからArrayを作成。

## メソッド

### init(&mut self, index: usize, value: T)
インデックス位置に値を初期化 (`ptr::write`使用)。

### as_ptr(&self) -> *mut T
生ポインタを取得。

### len(&self) -> usize
要素数を取得。

### as_slice(&self) -> &mut [T]
ミュータブルスライスに変換。

### get(&self, index: usize) -> &mut T
インデックスでミュータブル参照を取得。

### clone(&self) -> Self
シャローコピー (ポインタのみコピー)。

### deepcopy(&self, dst: &mut Array<U>)
ディープコピー (要素をコピー)。`Copy`トレイト必須。

### free(self)
メモリ解放 (現在は空実装)。

## Index実装
```rust
let value = arr[0];      // 読み取り
arr[0] = new_value;      // 書き込み
```

## 使用例
```rust
// 作成と初期化
let mut arr = Array::<u32>::new(10);
for i in 0..10 {
    arr.init(i, i as u32);
}

// アクセス
arr[0] = 100;
let v = arr[0];

// スライスとして使用
for elem in arr.as_slice() {
    println!("{}", elem);
}

// クローン (シャロー)
let arr2 = arr.clone();

// ディープコピー
let mut arr3 = Array::<u32>::new(10);
arr.deepcopy(&mut arr3);
```

## 注意点
- `clone()`はシャローコピー (同じメモリを指す)
- `init()`は未初期化メモリへの書き込みに使用
- `free()`は未実装 (リニアアロケーター)

## 関連ファイル
- `heap.rs`: メモリ確保元
- `ring.rs`: リングバッファ (Arrayを使用)
