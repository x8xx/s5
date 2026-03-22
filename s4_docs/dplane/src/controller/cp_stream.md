# dplane/src/controller/cp_stream.rs

## 概要
コントロールプレーンからのTCP接続を処理。

## 関数

### stream_handler(stream: TcpStream, table_list: Array<RwLock<Table>>) -> Result<(), Error>
TCP接続ハンドラ。

**処理ループ:**
1. リクエストを受信 (1024バイト)
2. コマンドを解析
3. 対応する処理を実行
4. レスポンスを送信

## コマンド

### Ping
接続確認。

**リクエスト:** `[cmd::RequestCmd::Ping]`
**レスポンス:** `[cmd::RequestCmd::Ping]`

### AddFlowEntry
フローエントリ追加。

**リクエスト:** `[cmd::RequestCmd::AddFlowEntry, table_id, ...entry_data]`
**レスポンス:** `[cmd::ResponseCmd::SuccessMessage]`

### ShowFlowEntry
フローエントリ表示 (未実装)。

**リクエスト:** `[cmd::RequestCmd::ShowFlowEntry, table_id]`

## 注意点
- 各接続で新しいHeapを作成 (512MB)
- エラー時は`ErrorMessage`を返却
- バッファサイズは固定1024バイト

## 関連ファイル
- `cmd.rs`: コマンド定義
- `table_controller.rs`: テーブル操作
- `cplane/main.go`: クライアント側
