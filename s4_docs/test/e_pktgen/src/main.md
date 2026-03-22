# test/e_pktgen/src/main.rs

## 概要
Ethernetパケットジェネレーター (e_pktgen) のエントリーポイント。DPDKベースの高性能パケット生成ツール。

## モジュール
```rust
mod dpdk;  // DPDK操作
mod cmd;   // コマンド定義
mod mode;  // 動作モード
```

## コマンドライン引数

### 必須オプション
- `-i, --interface`: インターフェース名 (例: "0000:06:00.0")
- `-m, --mode`: 動作モード ("shell" or "interactive")

### 使用例
```bash
# シェルモード
./e_pktgen -l 0-3 -n 4 -- -i 0000:06:00.0 -m shell -- count 1000

# インタラクティブモード
./e_pktgen -l 0-3 -n 4 -- -i 0000:06:00.0 -m interactive
```

## 動作モード

### shell
コマンドラインからコマンドを実行。バッチ処理向け。

### interactive
対話的なCLI。リアルタイム操作向け。

## 処理フロー
1. DPDK初期化
2. コマンドライン引数パース
3. シェル引数分離 (`--` 以降)
4. モードに応じてメイン処理
5. DPDKクリーンアップ

## 関連ファイル
- `dpdk/`: DPDK操作
- `cmd/`: コマンド実装
- `mode/`: モード実装
