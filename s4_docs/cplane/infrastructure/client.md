# cplane/infrastructure/client.go

## 概要
データプレーンと通信するためのTCPクライアント実装。

## 構造体

### Client
```go
type Client struct {
    addr string      // 接続先アドレス (host:port)
    conn net.Conn    // TCPコネクション
}
```

## 関数

### NewClient(addr string) *Client
新しいClientインスタンスを作成。

**引数:**
- `addr`: 接続先アドレス (例: "127.0.0.1:8888")

**戻り値:**
- `*Client`: 新しいClientインスタンス

### (client *Client) Connect() error
データプレーンへTCP接続を確立。

**戻り値:**
- `error`: 接続エラー (成功時はnil)

### (client *Client) Close()
TCP接続をクローズ。

### (client *Client) Send(buf []byte) ([]byte, error)
データを送信し、レスポンスを受信。

**引数:**
- `buf`: 送信するバイト列

**戻り値:**
- `[]byte`: 受信したレスポンス
- `error`: 通信エラー

**注意点:**
- レスポンスバッファは固定1024バイト
- 送信したbufの一部を返す実装バグあり (r_bufではなくbuf[:n]を返すべき)

## 通信プロトコル
同期的なリクエスト/レスポンスモデル:
1. `Write()`: コマンドを送信
2. `Read()`: レスポンスを受信
3. 受信バイト数分のデータを返却

## 使用例
```go
client := NewClient("127.0.0.1:8888")
if err := client.Connect(); err != nil {
    panic(err)
}
defer client.Close()

request := make([]byte, 1024)
request[0] = 0x01  // Ping
response, err := client.Send(request)
```

## 制限事項
- 固定バッファサイズ (1024バイト)
- 非同期通信未対応
- 再接続ロジックなし
- 接続タイムアウト設定なし

## 関連ファイル
- `main.go`: エントリーポイント
- `s4/dplane/src/controller/cp_stream.rs`: サーバー側ハンドラ
