# dplane/src/parser/header.rs

## 概要
パケットヘッダーとフィールドの構造定義。ビットレベルのフィールドアクセスを提供。

## 構造体

### Header
ヘッダー定義。

```rust
pub struct Header {
    pub fields: Array<Field>,           // 全フィールド
    pub used_fields: Array<Field>,      // キャッシュキー用フィールド
    pub parse_fields: Array<Field>,     // パース判定用フィールド
    pub l2_key_fields: Array<Field>,    // L2キャッシュキー用フィールド

    pub fields_len: usize,
    pub used_fields_len: usize,
    pub parse_fields_len: usize,
    pub l2_key_fields_len: usize,
}
```

### Field
フィールド定義。ビットレベルの位置とマスクを保持。

```rust
pub struct Field {
    pub start_byte_pos: usize,   // 開始バイト位置
    pub start_bit_mask: u8,      // 開始バイトのビットマスク
    pub end_byte_pos: usize,     // 終了バイト位置
    pub end_bit_mask: u8,        // 終了バイトのビットマスク
}
```

## コンストラクタ

### Header::new(field_len_list, used_field_index_list, parse_field_index_list)
ヘッダー定義を作成。

**引数:**
- `field_len_list`: 各フィールドのビット長
- `used_field_index_list`: キャッシュキー用フィールドインデックス
- `parse_field_index_list`: パース判定用フィールドインデックス

### Field::new(pre_field: &Field, field_bit_size: u16) -> Self
前フィールドに続く新フィールドを作成。

**ビットマスク計算:**
- 8ビット境界: `0xff`
- 部分ビット: `[128, 192, 224, 240, 248, 252, 254]`

## メソッド

### field.get() -> (start_byte, start_mask, end_byte, end_mask)
フィールド情報をタプルで取得。

### field.copy_ptr_value(base_offset, src, dst) -> isize
パケットからフィールド値をコピー。

**戻り値:** コピーしたバイト数

### field.cmp_pkt(pkt, hdr_offset, value, end_bit_mask) -> bool
パケットとフィールド値を比較 (完全一致)。

### field.cmp_pkt_ge_value(pkt, hdr_offset, value, end_bit_mask) -> bool
パケット値 >= フィールド値 を判定。

### field.cmp_pkt_le_value(pkt, hdr_offset, value, end_bit_mask) -> bool
パケット値 <= フィールド値 を判定。

## 使用例 (Ethernet)
```rust
// Ethernetヘッダー: dst(48bit), src(48bit), type(16bit)
let eth = Header::new(
    &[48, 48, 16],  // フィールドビット長
    &[0],           // dst MACをキャッシュキーに使用
    &[2],           // EtherTypeでパース判定
);

// フィールドアクセス
let dst_mac = &eth.fields[0];
assert_eq!(dst_mac.start_byte_pos, 0);
assert_eq!(dst_mac.end_byte_pos, 5);
```

## テスト
`test_field_new`: フィールド境界計算の検証
`test_copy_ptr_value`: 値コピーの検証
`test_cmp_pkt`: パケット比較の検証
`test_cmp_pkt_ge_value`: 大なりイコール比較の検証
`test_cmp_pkt_le_value`: 小なりイコール比較の検証

## 関連ファイル
- `parse_result.rs`: パース結果
- `pipeline/table.rs`: テーブルルックアップ
- `cache/cache.rs`: キャッシュキー生成
