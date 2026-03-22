# cplane/main.go

## 概要
コントロールプレーンのエントリーポイント。データプレーンへのTCP接続を確立し、コマンドを送信する。

## 依存関係
- `switch_cp/infrastructure`: TCPクライアント実装

## 機能

### main()
1. TCPクライアントを作成 (デフォルト: `127.0.0.1:8888`)
2. データプレーンに接続
3. コマンドバッファを作成・送信
4. レスポンスを受信・表示
5. 接続をクローズ

## プロトコル
バイナリプロトコルでデータプレーンと通信:
- `buf[0]`: コマンドコード
- `buf[1:]`: コマンドデータ

### コマンドコード例
```go
buf[0] = 97  // 'a' - AddFlowEntry相当
buf[1] = 100 // テーブルID等
```

## 使用例
```go
client := infrastructure.NewClient("127.0.0.1:8888")
client.Connect()
buf := make([]byte, 1024)
buf[0] = 97  // コマンド
r_buf, err := client.Send(buf)
client.Close()
```

## 現状と課題
- 現在はハードコードされたテストコマンドのみ
- 本格的なCLI/APIインターフェースは未実装
- エラーハンドリングは基本的なpanicのみ

## 関連ファイル
- `infrastructure/client.go`: TCPクライアント実装
- `s4/dplane/src/controller/cp_stream.rs`: データプレーン側ハンドラ
