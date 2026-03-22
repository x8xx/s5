# dplane/src/pipeline/table.rs

## 概要
フローテーブルの実装。エントリの管理、検索を担当。

## 型定義
```rust
type HeaderID = usize;
type FieldID = usize;
type MatchField = (HeaderID, FieldID);
```

## 列挙型

### MatchKind
マッチタイプ。

```rust
pub enum MatchKind {
    Exact,  // 完全一致
    Lpm,    // Longest Prefix Match
}
```

### Tree
検索ツリー。

```rust
pub enum Tree {
    Radix(RadixTree),  // LPM用
    Avl(AvlTree),      // Exact用
}
```

## 構造体

### Table
フローテーブル。

```rust
pub struct Table {
    pub tree: Tree,                              // 検索ツリー
    pub tree_key_index: usize,                   // ツリー検索キー
    pub keys: Array<(MatchField, MatchKind)>,   // マッチキー定義
    pub default_entry: FlowEntry,                // デフォルトエントリ
    pub header_list: Array<Header>,              // ヘッダー定義参照
}
```

### FlowEntry
フローエントリ。

```rust
pub struct FlowEntry {
    pub values: Array<MatchFieldValue>,  // マッチ値
    pub priority: u8,                    // 優先度
    pub action: ActionSet,               // アクション
}
```

### MatchFieldValue
マッチフィールド値。

```rust
pub struct MatchFieldValue {
    pub value: Option<Array<u8>>,  // None = any
    pub prefix_mask: u8,           // LPM用プレフィックスマスク
}
```

### ActionSet
アクションセット。

```rust
pub struct ActionSet {
    pub action_id: u8,              // アクションID
    pub action_data: Array<i32>,    // アクションパラメータ
}
```

## コンストラクタ

### Table::new(table_conf: &DpConfigTable, header_list: Array<Header>) -> Self
設定からテーブルを作成。

**処理:**
1. キー定義を構築
2. デフォルトエントリを作成
3. ツリータイプを決定 (LPM -> Radix, Exact -> AVL)

## メソッド

### search(&self, pkt: *const u8, parse_result: &ParseResult) -> &FlowEntry
パケットに一致するエントリを検索。

**アルゴリズム:**
1. ツリーキーで候補エントリを取得
2. 各候補に対して全キーをチェック
3. LPMの場合はプレフィックス長で比較
4. 優先度が高いエントリを選択
5. マッチなしの場合はデフォルトエントリ

### insert(&mut self, entry: FlowEntry)
エントリを追加。

### delete(&mut self, entry_id: usize)
エントリを削除 (未実装)。

## 検索フロー
```
1. tree.search(pkt[field]) -> 候補エントリリスト
2. for each 候補:
   - 全キーをマッチング
   - any/exact/lpmチェック
   - 優先度比較
3. return 最高優先度エントリ or default
```

## テスト
- `test_table_search`: テーブル検索の検証
  - Ethernet dst MAC + IPv4 dst のマルチキー
  - LPMマッチング
  - anyマッチング

## 関連ファイル
- `tree/avl_tree.rs`: AVLツリー
- `tree/radix_tree.rs`: Radixツリー
- `controller/table_controller.rs`: エントリ追加
