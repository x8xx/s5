# S5: Super Speed Software Switch Second

S4 (Super Speed Software Switch) の後継プロジェクト

## データプレーン (dplane)

```bash
cd dplane
cargo run      # 実行
cargo build    # ビルド
cargo test     # テスト
```

## 参考ドキュメント

### s4_docs/
S4の設計・実装ドキュメント。

| ディレクトリ | 内容 |
|-------------|------|
| `s4_docs/cplane/` | コントロールプレーン実装詳細 |
| `s4_docs/dplane/` | データプレーン実装詳細 |
| `s4_docs/test/` | テストツール実装詳細 |

### s4_src/
S4のソースコード。

| ディレクトリ | 内容 |
|-------------|------|
| `s4_src/cplane/` | コントロールプレーン (Go) |
| `s4_src/dplane/` | データプレーン (Rust) |
| `s4_src/test/` | テストツール |

## 結合テスト実行環境 (QEMU VM)

`test/vm/` にQEMU + Ubuntu VMベースの結合テスト環境を用意。

### 必要条件

```bash
# macOS
brew install qemu cdrtools

# Linux
sudo apt install qemu-system-x86 qemu-utils genisoimage  # amd64
sudo apt install qemu-system-arm qemu-utils genisoimage  # arm64
```

### セットアップ

```bash
cd test/vm

# SSH鍵を生成 (初回のみ)
make setup-ssh
# 出力された公開鍵を cloud-init/user-data の ssh_authorized_keys に追加

# Ubuntuイメージをダウンロード
make download
```

### 使い方

```bash
cd test/vm

make start      # VM起動
make ssh        # SSH接続
make stop       # VM停止
make status     # VM状態確認
make clean      # VMイメージ削除
```

### VM設定

| 項目 | 値 |
|------|-----|
| OS | Ubuntu 24.04 Server |
| メモリ | 1GB |
| CPU | 4コア |
| ディスク | 20GB |
| SSH | localhost:2222 |

### VM内でのDPDKセットアップ

```bash
# VM内で実行
sudo bash /path/to/scripts/setup-dpdk.sh
```

## テスト

### テストツール

| ツール | 説明 |
|--------|------|
| `test/e_pktgen/` | L2フレーム送受信ツール |
| `test/scripts/` | 自動テストスクリプト |

### VM上でのテスト実行

```bash
# VM内でビルド
cd ~/dplane && cargo build
cd ~/e_pktgen && cargo build

# テストスクリプト実行
sudo DPLANE_BIN=~/dplane/target/debug/s5-dplane \
     PKTGEN_BIN=~/e_pktgen/target/debug/e_pktgen \
     bash ~/0001_veth_forwarding.sh
```

### テストスクリプト一覧

| スクリプト | 説明 |
|-----------|------|
| `0001_veth_forwarding.sh` | vethペアを使用した基本パケット転送テスト |
