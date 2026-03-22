# S5 テスト環境

## 概要

S5データプレーン（dplane）とパケットジェネレータ（e_pktgen）の結合テスト環境。

### コンポーネント

| ディレクトリ | 説明 |
|-------------|------|
| `vm/` | QEMU VM環境（Ubuntu 24.04） |
| `e_pktgen/` | L2フレームジェネレータ |

## 必要条件

### VM環境
- QEMU（`test/vm/README.md`参照）
- Rust nightly（edition 2024使用）

### ビルド依存
- `nix` クレート（AF_PACKETソケット）
- `clap` クレート（CLI引数パース）
- `libc` クレート

## ビルド

### VM上でのビルド

```bash
# VM起動
cd test/vm
make start
make ssh

# VM内でRustセットアップ（初回のみ）
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source ~/.cargo/env
rustup install nightly
rustup default nightly

# ビルド
cd ~/dplane && cargo build
cd ~/e_pktgen && cargo build
```

## テスト環境セットアップ

### vethペアの作成

```bash
# 入力側ペア: veth0 <-> veth1
sudo ip link add veth0 type veth peer name veth1
sudo ip link set veth0 up
sudo ip link set veth1 up

# 出力側ペア: veth2 <-> veth3
sudo ip link add veth2 type veth peer name veth3
sudo ip link set veth2 up
sudo ip link set veth3 up

# 確認
ip link show veth0 veth1 veth2 veth3
```

### パケットフロー

```
e_pktgen → veth0 ─┐
                  │ (vethペア)
          veth1 ←─┘
            │
            ▼
        [dplane]
            │
            ▼
          veth2 ─┐
                 │ (vethペア)
          veth3 ←┘ → tcpdump
```

## テスト実行

### 1. dplane起動

```bash
# veth1から受信 → veth2に転送
sudo ~/dplane/target/debug/s5-dplane --rx veth1 --tx veth2 &
```

出力例:
```
Starting dplane: veth1 -> veth2
RX interface veth1 index: 6
TX interface veth2 index: 9
Forwarding packets...
```

### 2. tcpdump起動（別ターミナル）

```bash
sudo tcpdump -i veth3 -n -e
```

### 3. パケット送信

```bash
sudo ~/e_pktgen/target/debug/e_pktgen --interface veth0 --count 3
```

出力例:
```
Sending 3 packets to interface veth0 (src: 00:00:00:00:00:01, dst: ff:ff:ff:ff:ff:ff)
Interface veth0 index: 7
Sent packet 1/3
Sent packet 2/3
Sent packet 3/3
Done.
```

### 4. 結果確認

tcpdumpで以下のようにパケットがキャプチャされれば成功:
```
00:00:00:00:00:01 > ff:ff:ff:ff:ff:ff, ethertype IPv4 (0x0800), length 60: ...
00:00:00:00:00:01 > ff:ff:ff:ff:ff:ff, ethertype IPv4 (0x0800), length 60: ...
00:00:00:00:00:01 > ff:ff:ff:ff:ff:ff, ethertype IPv4 (0x0800), length 60: ...
3 packets captured
```

## コマンドリファレンス

### dplane

```bash
s5-dplane --rx <INTERFACE> --tx <INTERFACE>
```

| オプション | 説明 |
|-----------|------|
| `--rx` | 受信インターフェース名 |
| `--tx` | 送信インターフェース名 |

### e_pktgen

```bash
e_pktgen --interface <INTERFACE> --count <N> [OPTIONS]
```

| オプション | デフォルト | 説明 |
|-----------|-----------|------|
| `-i, --interface` | (必須) | 送信先インターフェース名 |
| `-c, --count` | 1 | 送信パケット数 |
| `--dst-mac` | ff:ff:ff:ff:ff:ff | 宛先MACアドレス |
| `--src-mac` | 00:00:00:00:00:01 | 送信元MACアドレス |

## 補足: tapデバイスについて

### tapデバイスとvethの違い

| 特性 | tap | veth |
|------|-----|------|
| 用途 | ユーザー空間⇔カーネル間トンネル | 仮想Ethernetケーブル |
| 動作 | fdを開いて読み書き | 通常のNICと同様 |
| AF_PACKET | 送信してもfd側に到達しない | ペアの反対側に到達する |

### tapデバイスが動作しない理由

tapデバイスはユーザー空間プログラムとカーネルネットワークスタック間のブリッジとして機能します：

```
[ユーザー空間]          [カーネル]
     │                     │
  open(/dev/net/tun)       │
     │                     │
  read(fd) ◄────────── tap1 (インターフェース)
  write(fd) ──────────►
```

AF_PACKETソケットでtapインターフェースにパケットを送信しても、それはカーネル側からの送信となり、ユーザー空間側（fd）には到達しません。

### tapデバイスを使用する場合

dplane自体がtapデバイスを作成・管理し、fdを直接読み書きする必要があります。これは将来の拡張として検討します。

### 現状の推奨テスト方法

vethペアを使用したテストを推奨します。vethペアは両端が直接接続された仮想Ethernetケーブルとして動作するため、AF_PACKETソケットでの送受信が正しく機能します。

## トラブルシューティング

### パケットが転送されない

1. インターフェースがUP状態か確認:
   ```bash
   ip link show veth0 veth1 veth2 veth3
   ```

2. dplaneがroot権限で実行されているか確認:
   ```bash
   sudo ~/dplane/target/debug/s5-dplane ...
   ```

3. インターフェースインデックスが正しいか確認:
   ```bash
   ip link show  # 各インターフェースのインデックスを確認
   ```

### Permission denied

AF_PACKETソケットにはroot権限が必要です:
```bash
sudo ./target/debug/s5-dplane ...
sudo ./target/debug/e_pktgen ...
```
