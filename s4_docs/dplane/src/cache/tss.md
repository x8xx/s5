# dplane/src/cache/tss.rs

## 概要
Tuple Space Search (TSS) によるL3キャッシュ実装。フィールド範囲によるタプル分類でキャッシュを組織化。

## 型定義

### TupleField
```rust
pub type TupleField = (MatchKind, Field);
```

### MatchKind (TSS)
```rust
pub enum MatchKind {
    Lpm,                           // LPMマッチ
    Exact(Array<u8>, Array<u8>),  // 範囲マッチ (start, end)
}
```

## 構造体

### TupleSpace
タプル空間全体。

```rust
pub struct TupleSpace<'a> {
    tuple_list: Array<Tuple>,           // タプルリスト
    tuple_len: usize,                   // タプル数
    tuple_hash_table: Array<&'a Tuple>, // タプルハッシュテーブル
    tuple_hash_seed: u32,               // ハッシュシード
}
```

### Tuple
個別タプル。

```rust
pub struct Tuple {
    fields: Array<TupleField>,          // フィールド定義
    cache: Array<RwLock<CacheElement>>, // キャッシュ配列
    hash: u16,                          // タプルハッシュ
    seed: u32,                          // キャッシュハッシュシード
}
```

### KeyStore
検索キー一時格納。

```rust
pub struct KeyStore {
    pub key: Array<u8>,
    pub key_len: usize,
}
```

### L3Cache
スレッド間共有用ラッパー。

```rust
pub struct L3Cache<'a> {
    pub l3_cache: *mut TupleSpace<'a>,
}
unsafe impl Send for L3Cache {}
unsafe impl Sync for L3Cache {}
```

## コンストラクタ

### TupleSpace::new(len: usize, tuple_hash_seed: u32) -> Self
タプル空間を作成。

### Tuple::new(fields: Array<TupleField>, cache_len: usize, seed: u32) -> Self
タプルを作成。

## メソッド

### TupleSpace::search(&self, pkt, key_store) -> Option<CacheData>
TSSでキャッシュを検索。

**アルゴリズム:**
1. 各タプルをイテレート
2. タプルのハッシュ関数でキーを計算
3. キャッシュエントリを参照
4. キーが一致すればデータを返却

### Tuple::hash_function(&self, pkt, key_store) -> Option<u16>
パケットからハッシュ値を計算。

**処理:**
1. 各フィールドをイテレート
2. LPM: 値をコピー
3. Exact: 範囲チェック後にコピー
4. MurmurHash3でハッシュ計算

### Tuple::tuple_hash(fields, seed) -> u16
タプル自体のハッシュを計算。

## TSSの概念
```
パケットフィールド値によって異なるタプルを選択:

Tuple 0: src_port in [0-1024], dst_port = any
Tuple 1: src_port in [1025-65535], protocol = TCP
Tuple 2: src_ip prefix /24, dst_ip prefix /16

パケット → タプル選択 → キャッシュルックアップ
```

## 関連ファイル
- `cache.rs`: キャッシュ要素
- `hash.rs`: ハッシュ関数
- `worker/cache.rs`: L3キャッシュ検索
