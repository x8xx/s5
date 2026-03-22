# dplane/src/parser/parse_result.rs

## 概要
パケットパース結果を格納する構造体定義。

## 構造体

### ParseResult
パース結果全体。

```rust
pub struct ParseResult {
    pub metadata: Array<u32>,          // メタデータ配列
    pub hdr_size: usize,               // ヘッダー合計サイズ
    pub header_list: Array<Header>,    // パースされたヘッダーリスト
}
```

### Header (parse_result)
個別ヘッダーの状態。

```rust
pub struct Header {
    pub is_valid: bool,  // ヘッダーが存在するか
    pub offset: u16,     // パケット先頭からのオフセット
}
```

## 列挙型

### Metadata
メタデータのインデックス定義。

```rust
pub enum Metadata {
    InPort = 0,  // 入力ポート番号
}
```

## 使用例
```rust
let mut parse_result = ParseResult {
    metadata: Array::new(1),
    hdr_size: 0,
    header_list: Array::new(4),  // Eth, IPv4, TCP, UDP
};

// 初期化
parse_result.metadata[Metadata::InPort as usize] = 1;
for i in 0..4 {
    parse_result.header_list[i].is_valid = false;
}

// パース後
// header_list[0] = { is_valid: true, offset: 0 }    // Ethernet
// header_list[1] = { is_valid: true, offset: 14 }   // IPv4
// header_list[2] = { is_valid: true, offset: 34 }   // TCP
// hdr_size = 54
```

## フィールド詳細

### metadata
パケットに関連するメタ情報。現在は入力ポートのみ。

### hdr_size
全ヘッダーの合計サイズ (バイト)。ペイロード開始位置の計算に使用。

### header_list
各プロトコルヘッダーの状態。インデックスはヘッダーID。

## 関連ファイル
- `parser.rs`: ParseResult生成
- `pipeline/pipeline.rs`: ParseResult参照
- `cache/cache.rs`: キャッシュキー生成
