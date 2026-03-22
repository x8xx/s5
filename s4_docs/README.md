# S4 Data Plane ドキュメント

## 概要

S4 (Super Speed Software Switch) はDPDK/Linuxをサポートする高性能ソフトウェアスイッチ。WASMベースのパーサー/パイプラインとマルチレベルキャッシュにより高速なパケット処理を実現。

## 全体アーキテクチャ

```
┌─────────────────────────────────────────────────────────────────────┐
│                          Control Plane (Go)                         │
│                              TCP接続                                │
└──────────────────────────────────┬──────────────────────────────────┘
                                   │
┌──────────────────────────────────▼──────────────────────────────────┐
│                          Data Plane (Rust)                          │
│                                                                     │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │                        Controller                            │   │
│  │   - ワーカー管理                                             │   │
│  │   - TCPサーバー (CP通信)                                     │   │
│  │   - テーブル操作                                             │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                                                                     │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │                     Packet Processing                        │   │
│  │                                                               │   │
│  │   [NIC] → [RX] → [Cache] → [Pipeline] → [TX] → [NIC]        │   │
│  │                                 │                             │   │
│  │                          [Cache Creater]                      │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                                                                     │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │                     Core (抽象化層)                          │   │
│  │   - memory: ヒープ/リング/ベクタ                             │   │
│  │   - network: インターフェース/パケットバッファ               │   │
│  │   - thread: スレッド管理                                     │   │
│  │   - runtime: WASMランタイム (Wasmer)                         │   │
│  └─────────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────────┘
```

## 処理フロー

### 1. 起動シーケンス

```
main()
  │
  ├─→ 環境初期化
  │     DPDK: EAL初期化 (Hugepages, lcores)
  │     Linux: 標準ヒープ初期化
  │
  ├─→ 設定読み込み (config.rs)
  │     - YAML: スイッチ設定
  │     - JSON: データプレーン設定
  │     - WASM: パーサー/パイプラインバイトコード
  │
  └─→ コントローラ起動 (controller.rs)
        │
        ├─→ データベース初期化
        │     - ヘッダーリスト作成
        │     - テーブルリスト作成
        │     - 初期フローエントリ登録
        │
        ├─→ キャッシュ初期化
        │     - L1キャッシュ (RXワーカーごと)
        │     - L2キャッシュ (Cacheワーカーごと)
        │     - LBF (Load Balancer Filter)
        │     - L3 TSS (Tuple Space Search)
        │
        ├─→ リングバッファ初期化
        │     - RX → Cache
        │     - RX/Cache → Pipeline
        │     - Pipeline → TX
        │     - Pipeline → CacheCreater
        │
        ├─→ ワーカー起動
        │     1. Pipeline Workers
        │     2. Cache Workers
        │     3. TX Workers
        │     4. RX Workers
        │     5. Cache Creater
        │
        └─→ TCPサーバー起動
              - コントロールプレーン接続受付
```

### 2. パケット処理フロー

```
                    パケット受信
                         │
                         ▼
┌─────────────────────────────────────────────────────────────────┐
│                      RX Worker                                   │
│   1. NICからパケット受信                                        │
│   2. パーサー実行 (WASM) → ヘッダー解析                         │
│   3. L1キャッシュチェック (ヘッダー完全一致)                    │
│      - HIT  → Pipeline Workerへ (キャッシュパイプライン)        │
│      - MISS → L2キー生成 → LBFでCache Worker選択 → Cache Workerへ│
└─────────────────────────────────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────────┐
│                     Cache Worker                                 │
│   1. LBFヒット時のみL2キャッシュチェック (抽出フィールド一致)   │
│      - HIT  → Pipeline Workerへ (キャッシュパイプライン)        │
│      - MISS → Pipeline Workerへ (フルパイプライン)              │
│   2. LBFミス時                                                   │
│      - Pipeline Workerへ (フルパイプライン)                     │
│   ※ L3 TSS検索は現在無効化されている                           │
└─────────────────────────────────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────────┐
│                    Pipeline Worker                               │
│   キャッシュHITの場合:                                          │
│     - キャッシュされたアクションを直接適用                      │
│   キャッシュMISSの場合:                                         │
│     1. テーブルルックアップ (WASM)                              │
│     2. アクション選択・実行                                     │
│     3. パケット変更                                             │
│     4. キャッシュデータ生成 → Cache Createrへ通知               │
│   5. 出力先決定 → TX Workerへ                                   │
└─────────────────────────────────────────────────────────────────┘
                         │
          ┌──────────────┴──────────────┐
          │                             │
          ▼                             ▼
┌─────────────────────┐    ┌─────────────────────────────────────┐
│     TX Worker       │    │          Cache Creater              │
│  パケット送信       │    │   - L1/L2キャッシュ更新             │
│  (NICへ)            │    │   - LBF更新                         │
└─────────────────────┘    └─────────────────────────────────────┘
```

### 3. キャッシュ階層

| レベル | 配置 | キー | 特徴 | 状態 |
|--------|------|------|------|------|
| L1 | RXワーカーごと | パケットヘッダー全体 | 高速、低ヒット率 | 有効 |
| L2 | Cacheワーカーごと | 抽出フィールド値 | 中速、中ヒット率 | 有効 |
| LBF | インターフェースごと | L2ハッシュ | L2キャッシュ存在フラグ | 有効 |
| L3 (TSS) | 全ワーカー共有 | タプルスペース | 低速、高ヒット率 | 無効化 |

### 4. ワーカー間通信

- **通信方式**: ロックフリーRingバッファ (MPMC)
- **負荷分散**: ラウンドロビン / LBF

```
                    L1 HIT
RX Workers ─────────────────────────┐
     │                              │
     │ L1 MISS                      │
     ▼                              ▼
[Cache Ring] ──→ Cache Workers ──→ [Pipeline Ring] ──→ Pipeline Workers
                                                            │
                                         ┌──────────────────┴──────────────────┐
                                         │                                     │
                                         ▼                                     ▼
                               [TX Ring] ──→ TX Workers           [Cache Creater Ring]
                                                                         │
                                                                         ▼
                                                                  Cache Creater
                                                              (L1/L2キャッシュ更新)
```

## モジュール構成

| モジュール | 役割 | 詳細 |
|-----------|------|------|
| `main.rs` | エントリーポイント | 環境初期化、設定読み込み、コントローラ起動 |
| `config.rs` | 設定パーサー | YAML/JSON設定ファイル解析 |
| `controller.rs` | メインコントローラ | ワーカー起動、TCPサーバー管理 |
| `core/` | プラットフォーム抽象化層 | DPDK/Linux両対応 |
| `parser/` | パケットパーサー | WASMベースのヘッダー解析 |
| `cache/` | キャッシュシステム | マルチレベルキャッシュ (L1/L2/L3) |
| `pipeline/` | パイプライン処理 | テーブルルックアップ、アクション実行 |
| `deparser/` | パケット再構築 | ヘッダー書き戻し |
| `worker/` | ワーカースレッド | RX/Cache/Pipeline/TX/CacheCreater |

## 設定ファイル

### スイッチ設定 (YAML)

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

### データプレーン設定 (JSON)

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

## コンパイル・実行

```bash
# DPDKモード
./s4dp -l 0-7 -n 4 -- -c config/switch_config.yml

# Linuxモード (開発/デバッグ用)
./s4dp -c config/switch_config.yml
```

## ディレクトリ構成

```
s4_docs/
├── README.md          # 本ファイル
├── cplane/            # コントロールプレーン実装詳細
├── dplane/            # データプレーン実装詳細
│   └── src/
│       ├── main.md
│       ├── config.md
│       ├── controller.md
│       ├── core.md
│       ├── parser.md
│       ├── cache.md
│       ├── pipeline.md
│       ├── deparser.md
│       └── worker.md
└── test/              # テストツール実装詳細
```
