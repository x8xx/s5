# S5 Integration Test VM

QEMU + Ubuntu cloud image を使用した結合テスト環境。

## 必要条件

### macOS
```bash
brew install qemu
```

### Linux
```bash
sudo apt install qemu-system-x86 qemu-utils genisoimage  # amd64
sudo apt install qemu-system-arm qemu-utils genisoimage   # arm64
```

## クイックスタート

```bash
cd test/vm

# SSH鍵を生成 (初回のみ)
make setup-ssh
# 出力された公開鍵を cloud-init/user-data の ssh_authorized_keys に追加

# Ubuntuイメージをダウンロード
make download

# VM起動
make start

# SSH接続
make ssh

# VM停止
make stop
```

## コマンド一覧

| コマンド | 説明 |
|----------|------|
| `make download` | Ubuntu cloud imageをダウンロード |
| `make start` | VMを起動 |
| `make stop` | VMを停止 |
| `make ssh` | VMにSSH接続 |
| `make status` | VM状態を確認 |
| `make clean` | VMイメージを削除 |
| `make clean-all` | 全イメージを削除 |
| `make setup-ssh` | SSH鍵ペアを生成 |

## VM設定

- **OS**: Ubuntu 24.04 Server
- **メモリ**: 1GB
- **CPU**: 4コア
- **ディスク**: 20GB
- **SSH**: localhost:2222

## VM内でのセットアップ

### DPDKインストール

```bash
# VM内で実行
sudo /vagrant/scripts/setup-dpdk.sh  # 共有フォルダ経由の場合
# または
sudo bash -c "$(curl -fsSL https://raw.githubusercontent.com/.../setup-dpdk.sh)"
```

### 手動でDPDKをインストール

```bash
# VM内で実行
cd /tmp
wget https://fast.dpdk.org/rel/dpdk-22.03.tar.xz
tar xf dpdk-22.03.tar.xz
cd dpdk-22.03
meson setup build --prefix=/opt/dpdk
cd build
ninja
sudo ninja install
```

## ファイル構成

```
test/vm/
├── Makefile              # VM操作コマンド
├── cloud-init/
│   ├── user-data         # cloud-init設定
│   └── meta-data         # インスタンスメタデータ
├── scripts/
│   └── setup-dpdk.sh     # DPDKセットアップスクリプト
├── images/               # (生成される) VMイメージ
└── README.md
```

## トラブルシューティング

### QEMUが起動しない

```bash
# macOS: HVFが有効か確認
sysctl kern.hv_support

# Linux: KVMが有効か確認
lsmod | grep kvm
```

### SSHに接続できない

VMの起動に1-2分かかります。`make status`で起動確認後、再試行してください。

```bash
# 直接接続を試す
ssh -o StrictHostKeyChecking=no -p 2222 s5@localhost
```
