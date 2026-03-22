# e_pktgen - Ethernet Packet Generator

## 概要
DPDKを使用した高性能パケットジェネレーター。S4スイッチのテストとベンチマークに使用。

## 機能
- 高速パケット生成 (DPDKベース)
- カウントベース/タイムベースの生成モード
- カスタムパケット生成 (ライブラリ経由)

## ディレクトリ構造
```
e_pktgen/
├── src/
│   ├── main.rs           # エントリーポイント
│   ├── dpdk.rs           # DPDKモジュール
│   │   ├── common.rs     # 初期化/クリーンアップ
│   │   ├── interface.rs  # NIC操作
│   │   ├── pktbuf.rs     # パケットバッファ
│   │   ├── memory.rs     # メモリ管理
│   │   └── thread.rs     # スレッド管理
│   ├── cmd.rs            # コマンドモジュール
│   │   ├── count.rs      # カウントモード
│   │   │   ├── gen.rs    # パケット生成
│   │   │   └── rx.rs     # パケット受信
│   │   └── time.rs       # タイムモード
│   │       ├── gen.rs    # パケット生成
│   │       └── rx.rs     # パケット受信
│   └── mode.rs           # モードモジュール
│       ├── shell.rs      # シェルモード
│       └── interactive.rs # インタラクティブモード
└── Cargo.toml
```

## ビルド
```bash
cargo build --release
```

## 使用方法

### シェルモード
```bash
# 1000パケット生成
./e_pktgen -l 0-3 -n 4 -- -i 0000:06:00.0 -m shell -- count 1000

# 10秒間生成
./e_pktgen -l 0-3 -n 4 -- -i 0000:06:00.0 -m shell -- time 10
```

### インタラクティブモード
```bash
./e_pktgen -l 0-3 -n 4 -- -i 0000:06:00.0 -m interactive
```

## コマンド

### count <num>
指定数のパケットを生成。

### time <seconds>
指定時間パケットを生成。

## 関連ファイル
- `testdata/l2switch/e_pktgen_conf.yml`: 設定例
- `testdata/l2switch/pktgen/`: パケット生成ライブラリ
