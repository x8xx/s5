# dplane/src/parser/parser.rs

## 概要
WASMベースのパケットパーサー。パケットを解析し、ヘッダー情報を抽出。

## 構造体

### Parser
```rust
pub struct Parser {
    runtime: Runtime,           // WASMランタイム
    runtime_args: RuntimeArgs,  // WASM関数引数
}
```

## コンストラクタ

### Parser::new(wasm: &[u8]) -> Self
WASMバイトコードからパーサーを作成。

**登録されるネイティブ関数:**
- `s4_sys_pkt_get_len`: パケット長取得
- `s4_sys_pkt_read`: パケットデータ読み取り
- `s4_sys_pkt_drop`: パケットドロップフラグ設定
- `s4_sys_extract_hdr`: ヘッダー抽出

## メソッド

### parse(&mut self, pkt: *mut u8, pkt_len: usize, parse_result: &mut ParseResult) -> bool
パケットを解析。

**引数:**
- `pkt`: パケットデータポインタ
- `pkt_len`: パケット長
- `parse_result`: 解析結果格納先

**戻り値:**
- `true`: 解析成功 (accept)
- `false`: 解析失敗 (drop)

**処理:**
1. `ParserArgs`構造体を作成
2. WASM関数`parse`を呼び出し
3. `is_accept`フラグを返却

## ParserArgs
WASMに渡される引数構造体。

```rust
struct ParserArgs {
    pkt: *mut u8,
    pkt_len: usize,
    parse_result: &mut ParseResult,
    is_accept: &mut bool,
}
```

## WASM関数インターフェース

### parse(args_ptr: i64)
WASMエクスポート関数。

**期待される動作:**
1. `args_ptr`から`ParserArgs`を取得
2. ネイティブAPIを使用してパケットを解析
3. `parse_result`にヘッダー情報を格納
4. `is_accept`にaccept/dropを設定

## 使用例
```rust
let parser = Parser::new(&parser_wasm_bytes);

let mut parse_result = ParseResult { ... };
if parser.parse(pkt_ptr, pkt_len, &mut parse_result) {
    // 解析成功 - パイプラインへ
} else {
    // 解析失敗 - ドロップ
}
```

## 関連ファイル
- `runtime_native_api.rs`: ネイティブAPI実装
- `parse_result.rs`: 結果構造体
- `header.rs`: ヘッダー定義
- `worker/rx.rs`: パーサー呼び出し元
