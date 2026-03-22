# dplane/src/pipeline/output.rs

## 概要
パケット出力先の定義。

## 列挙型

### Output
出力先タイプ。

```rust
pub enum Output {
    Port(u32),    // 特定ポートに出力
    All,          // 全ポートに出力 (フラッディング)
    Controller,   // コントローラに送信
    Drop,         // パケットをドロップ
}
```

## 使用例
```rust
let mut output = Output::Drop;

// パイプライン内で設定
output = Output::Port(2);  // ポート2に出力

// ワーカーで処理
match output {
    Output::Port(port) => tx_ring[port].enqueue(pkt),
    Output::All => { /* 全ポートに送信 */ },
    Output::Controller => { /* CPUに送信 */ },
    Output::Drop => pkt.free(),
}
```

## 関連ファイル
- `pipeline/pipeline.rs`: Output設定
- `pipeline/runtime_native_api.rs`: WASMからの設定
- `worker/pipeline.rs`: Outputに基づく送信
