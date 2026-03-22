# dplane/src/main.rs

## 概要
データプレーンのエントリーポイント。DPDK/Linux環境の初期化、設定読み込み、コントローラ起動を行う。

## モジュール宣言
```rust
mod core;       // プラットフォーム抽象化層
mod config;     // 設定パーサー
mod controller; // メインコントローラ
mod parser;     // パケットパーサー
mod cache;      // キャッシュシステム
mod pipeline;   // パイプライン処理
mod deparser;   // パケット再構築
mod worker;     // ワーカースレッド
```

## main()
1. **環境初期化**
   - DPDK: `core::helper::dpdk::init()` - EALを初期化
   - Linux: 引数インデックス0から開始

2. **引数パース**
   - DPDKはEAL引数を消費するため、`switch_args_start_index`以降がスイッチ引数

3. **設定読み込み**
   - `config::parse_switch_args()`: YAML設定ファイルをパース

4. **コントローラ起動**
   - `controller::start_controller()`: ワーカー起動、TCPサーバー開始

5. **クリーンアップ** (DPDKのみ)
   - `core::helper::dpdk::cleanup()`: EAL終了処理

## コンパイル時フィーチャー
```rust
#[cfg(feature="dpdk")]   // 本番環境: DPDKバックエンド
#[cfg(feature="linux")]  // 開発環境: Linuxバックエンド
```

## 使用例
```bash
# DPDKモード
./s4dp -l 0-7 -n 4 -- -c config/switch_config.yml

# Linuxモード (debug)
./s4dp -c config/switch_config.yml
```

## 関連ファイル
- `config.rs`: 設定ファイルパーサー
- `controller.rs`: メインコントローラ
- `core/helper/dpdk.rs`: DPDK初期化
- `core/helper/linux.rs`: Linux初期化
