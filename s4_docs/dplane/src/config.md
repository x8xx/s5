# dplane/src/config.rs

## 概要
スイッチ設定ファイル (YAML) とデータプレーン設定 (JSON) をパースして構造化データに変換。

## 構造体

### SwitchConfig
スイッチ全体の設定を保持。

```rust
pub struct SwitchConfig {
    pub dataplane: DpConfig,           // データプレーン設定
    pub parser_wasm: Vec<u8>,          // パーサーWASMバイトコード
    pub pipeline_wasm: Vec<u8>,        // パイプラインWASMバイトコード
    pub listen_address: String,         // TCPリッスンアドレス
    pub cache_core_num: u8,            // キャッシュワーカー数
    pub pipeline_core_num: u8,         // パイプラインワーカー数
    pub l1_cache_size: usize,          // L1キャッシュサイズ
    pub l2_cache_size: usize,          // L2キャッシュサイズ
    pub l3_cache_tuple_size: usize,    // L3 TSSサイズ
    pub interface_configs: Vec<InterfaceConfig>,  // インターフェース設定
    pub initial_table_data: Vec<u8>,   // 初期フローエントリ
}
```

### InterfaceConfig
ネットワークインターフェースごとの設定。

```rust
pub struct InterfaceConfig {
    pub if_name: String,        // インターフェース名 (例: "0000:06:00.0")
    pub cache_core_num: u8,     // キャッシュコア数
    pub rx_queue: u16,          // RXキュー数
    pub tx_queue: u16,          // TXキュー数
}
```

### DpConfig
データプレーン設定 (JSON由来)。

```rust
pub struct DpConfig {
    pub headers: Vec<DpConfigHeader>,  // ヘッダー定義
    pub header_max_size: usize,        // 最大ヘッダーサイズ
    pub tables: Vec<DpConfigTable>,    // テーブル定義
}
```

### DpConfigHeader
パケットヘッダーの構造定義。

```rust
pub struct DpConfigHeader {
    pub fields: Vec<u16>,        // フィールドビット長
    pub used_fields: Vec<u16>,   // キャッシュキー用フィールドインデックス
    pub parse_fields: Vec<u16>,  // パース判定用フィールドインデックス
}
```

### DpConfigTable
フローテーブルの構造定義。

```rust
pub struct DpConfigTable {
    pub keys: Vec<DpConfigTableKey>,  // マッチキー定義
    pub tree_key_index: usize,        // ツリー検索用キーインデックス
    pub default_action_id: u64,       // デフォルトアクションID
    pub max_size: u64,                // 最大エントリ数
}
```

### DpConfigTableKey
テーブルキーの定義。

```rust
pub struct DpConfigTableKey {
    pub match_kind: String,  // "exact" or "lpm"
    pub header_id: u64,      // ヘッダーID
    pub field_id: u64,       // フィールドID
}
```

## 関数

### parse_switch_args(args: &[String]) -> SwitchConfig
コマンドライン引数から設定を読み込む。

**処理フロー:**
1. `-c/--config` オプションからYAMLパスを取得
2. YAMLファイルをパース
3. `general` セクションから一般設定を読み込み
4. `dataplane_config_path` からJSON設定を読み込み
5. WASMファイルをバイナリ読み込み
6. `interfaces` セクションからインターフェース設定を読み込み

## YAML設定ファイル構造
```yaml
general:
  dataplane_config_path: "path/to/dataplane.json"
  parser_wasm_path: "path/to/parser.wasm"
  pipeline_wasm_path: "path/to/pipeline.wasm"
  initial_table_data_path: "path/to/table.bin"
  listen_address: "127.0.0.1:8888"
  cache_core_num: 4
  pipeline_core_num: 4
  l1_cache_size: 65536
  l2_cache_size: 65536
  l3_cache_tuple_size: 10000

interfaces:
  "0000:06:00.0":
    cache_core_num: 2
    rx_queue: 4
    tx_queue: 4
```

## JSON設定ファイル構造 (dataplane.json)
```json
{
  "headers": [
    {
      "fields": [48, 48, 16],
      "used_fields": [0],
      "parse_fields": [2]
    }
  ],
  "header_max_size": 128,
  "tables": [
    {
      "keys": [
        {"match_kind": "exact", "header_id": 0, "field_id": 0}
      ],
      "tree_key_index": 0,
      "default_action_id": 0,
      "max_size": 10000
    }
  ]
}
```

## 依存関係
- `getopts`: コマンドライン引数パース
- `yaml_rust`: YAMLパーサー
- `serde`/`serde_json`: JSONパーサー

## 関連ファイル
- `main.rs`: エントリーポイント
- `controller.rs`: 設定を使用してワーカーを起動
