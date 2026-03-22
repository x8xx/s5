# dplane/src/cache/cache.rs

## 概要
キャッシュ要素の定義。キーと対応するフローエントリを保持。

## 型定義

### CacheData
```rust
pub type CacheData = Array<*const FlowEntry>;
```
各テーブルのルックアップ結果 (FlowEntryへのポインタ配列)。

## 構造体

### CacheElement
キャッシュエントリ。

```rust
pub struct CacheElement {
    pub key: Array<u8>,    // キャッシュキー
    pub key_len: isize,    // キー長
    pub data: CacheData,   // キャッシュデータ
}
```

### CacheRelation
キャッシュ関連付け (未使用)。

```rust
pub struct CacheRelation {
    pub l1_cache: Array<*mut CacheElement>,
    pub l2_cache: Array<*mut CacheElement>,
}
```

## メソッド

### cmp_ptr_key(&self, ptr_key: *const u8, key_len: isize) -> bool
キーを比較。

**処理:**
1. 長さチェック
2. バイト単位で比較

## 使用例
```rust
let cache_element = CacheElement {
    key: Array::new(128),
    key_len: 0,
    data: Array::new(table_count),
};

// キャッシュルックアップ
if cache_element.cmp_ptr_key(pkt_ptr, header_size) {
    // ヒット
    let flow_entry = cache_element.data[table_id];
}
```

## テスト
- `test_cmp_ptr_key`: キー比較の検証

## 関連ファイル
- `hash.rs`: ハッシュ関数
- `tss.rs`: Tuple Space Search
- `worker/rx.rs`: L1キャッシュルックアップ
- `worker/cache.rs`: L2キャッシュルックアップ
