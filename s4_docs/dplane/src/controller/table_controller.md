# dplane/src/controller/table_controller.rs

## 概要
フローテーブルへのエントリ追加を担当。

## 関数

### add_flow_entry(table: &mut RwLock<Table>, request_buffer: &[u8])
バイナリリクエストからフローエントリを追加。

**リクエストバッファフォーマット:**
```
value_count(1) | (value_len(1) | value(N) | prefix_mask(1))* |
priority(1) | action_id(1) | action_data_len(1) | action_data(N)
```

**処理フロー:**
1. `value_count`を読み取り
2. 各値を読み取り:
   - `value_len == 0`: any
   - `value_len > 0`: 値とprefix_mask
3. `priority`を読み取り
4. `action_id`を読み取り
5. `action_data`を読み取り (i32配列に変換)
6. FlowEntryを作成
7. テーブルに挿入

### show_flow_entry(table: &Table) -> String
フローエントリを表示 (未実装)。

## バイナリフォーマット例
```
// MAC 01:02:03:04:05:06、any、priority 2、action_id 1、data [10,20,30,40,50,60]
[
    2,              // value_count
    6,              // value_len[0]
    1,2,3,4,5,6,    // value[0]
    0xff,           // prefix_mask[0]
    0,              // value_len[1] (any)
    2,              // priority
    1,              // action_id
    6,              // action_data_len
    10,20,30,40,50,60  // action_data
]
```

## テスト
- `test_add_flow_entry`: エントリ追加の検証

## 関連ファイル
- `cp_stream.rs`: TCPリクエストハンドラ
- `pipeline/table.rs`: テーブル実装
