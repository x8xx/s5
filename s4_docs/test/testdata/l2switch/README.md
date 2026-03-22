# L2 Switch Test Data

## 概要
L2スイッチの参照実装。Parser/PipelineのWASMモジュールと設定ファイルを含む。

## ディレクトリ構造
```
l2switch/
├── parser/
│   └── parser.rs         # パーサーWASMソース
├── pipeline/
│   └── pipeline.rs       # パイプラインWASMソース
├── pktgen/
│   └── src/lib.rs        # パケット生成ライブラリ
├── dataplane.json        # データプレーン設定
├── switch_config.yml     # スイッチ設定
├── e_pktgen_conf.yml     # パケットジェネレーター設定
├── gen_table_entry.py    # テーブルエントリ生成スクリプト
└── Makefile              # ビルド自動化
```

## Parser (parser.rs)

### parse(parser_args_ptr: i64) -> bool
パケットをパース。

**処理:**
1. パケット長取得
2. Ethernetヘッダー (14バイト) を抽出
3. パケット長が14バイト未満ならfalse

### ヘッダー構造
- Header 0: Ethernet (dst_mac, src_mac, ethertype)

## Pipeline (pipeline.rs)

### run_pipeline(pipeline_args: i64)
パイプライン処理を実行。

**アクション:**
| action_id | 動作 | パラメータ |
|-----------|------|-----------|
| 0 | 特定ポート出力 | port番号 |
| 1 | フラッディング | なし |
| 2+ | ドロップ | なし |

## 設定ファイル

### dataplane.json
```json
{
  "headers": [
    {
      "fields": [48, 48, 16],      // dst(6B), src(6B), type(2B)
      "used_fields": [0],          // dstをマッチに使用
      "parse_fields": []
    }
  ],
  "header_max_size": 14,
  "tables": [
    {
      "keys": [{"match_kind": "exact", "header_id": 0, "field_id": 0}],
      "tree_key_index": 0,
      "default_action_id": 2,      // デフォルト: drop
      "max_size": 10000
    }
  ]
}
```

### switch_config.yml
```yaml
general:
  dataplane_config_path: "dataplane.json"
  parser_wasm_path: "parser.wasm"
  pipeline_wasm_path: "pipeline.wasm"
  ...
```

## ビルド
```bash
make
```

WASMファイル生成:
- `parser.wasm`
- `pipeline.wasm`

## テスト実行
```bash
# S4スイッチ起動
cd s4/dplane
cargo run --release -- -l 0-15 -n 4 -- -c ../test/testdata/l2switch/switch_config.yml

# パケットジェネレーター
cd s4/test/e_pktgen
cargo run --release -- -l 16-19 -n 4 -- -i net_tap0 -m shell -- count 10000
```
