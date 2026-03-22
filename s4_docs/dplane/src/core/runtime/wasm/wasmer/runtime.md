# dplane/src/core/runtime/wasm/wasmer/runtime.rs

## 概要
Wasmer WASMランタイムのラッパー。Parser/PipelineのWASMモジュールを実行。

## 型定義

### RuntimeArgs
```rust
pub type RuntimeArgs = Array<wasmer::Value>;
```
WASM関数の引数配列。

## 構造体

### Runtime
```rust
pub struct Runtime {
    pub store: wasmer::Store,
    pub module: wasmer::Module,
    pub import_object: wasmer::ImportObject,
    pub instance: wasmer::Instance,
}
```

## マクロ

### new_runtime_args!(argc)
引数配列を作成。

```rust
let args = new_runtime_args!(3);  // 3引数用
```

### new_runtime!(wasm, { native_funcs })
WASMモジュールからRuntimeを作成。

```rust
let runtime = new_runtime!(
    wasm_bytes,
    {
        "native_func_name" => native_func,
    }
);
```

**処理:**
1. Store作成 (LLVMコンパイラ使用時)
2. WASMバイナリからモジュール作成
3. リニアメモリ作成
4. ネイティブ関数をインポートオブジェクトに登録
5. インスタンス作成

### set_runtime_arg_i64!(args, index, value)
i64引数を設定。

### set_runtime_arg_i32!(args, index, value)
i32引数を設定。

### call_runtime!(runtime, func_name, args)
WASM関数を呼び出し。

### call_runtime_i32!(runtime, func_name, args)
WASM関数を呼び出し、i32結果を取得。

## コンパイラ設定
```rust
#[cfg(feature="wasmer_llvm")]
let store = {
    let llvm_compiler = wasmer_compiler_llvm::LLVM::default();
    Store::new(&Universal::new(llvm_compiler).engine())
};
```

LLVMコンパイラを使用することで、高性能なJITコンパイルを実現。

## ネイティブ関数登録
```rust
import_object.register("env", {
    "__linear_memory" => linear_memory,
    "s4_sys_func" => native_func,
});
```

## 使用例 (Parser)
```rust
let runtime = new_runtime!(
    parser_wasm,
    {
        "s4_sys_pkt_read" => pkt_read,
        "s4_sys_extract_hdr" => extract_hdr,
    }
);

let args = new_runtime_args!(1);
set_runtime_arg_i64!(args, 0, ptr as i64);
call_runtime!(runtime, "parse", args);
```

## 関連ファイル
- `parser/parser.rs`: パーサーWASM
- `pipeline/pipeline.rs`: パイプラインWASM
- `parser/runtime_native_api.rs`: パーサー用ネイティブAPI
- `pipeline/runtime_native_api.rs`: パイプライン用ネイティブAPI
