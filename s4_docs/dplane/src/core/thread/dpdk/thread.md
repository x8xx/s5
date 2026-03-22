# dplane/src/core/thread/dpdk/thread.rs

## 概要
DPDKのlcoreを使用したスレッドスポーン。各lcoreで関数を実行。

## グローバル変数
```rust
static mut CURRENT_LCORE_ID: u32 = u32::MIN;
```
次に使用するlcore ID。

## 関数

### thread_init()
lcore管理を初期化。マスターlcoreの次のlcoreIDを取得。

**処理:**
```rust
CURRENT_LCORE_ID = rte_get_next_lcore(u32::MIN, 1, 0);
```

### spawn(func: extern "C" fn(*mut c_void) -> i32, args: *mut c_void) -> bool
関数を新しいlcoreで実行。

**引数:**
- `func`: 実行する関数 (extern "C"必須)
- `args`: 関数引数 (ポインタ)

**戻り値:**
- `true`: 成功
- `false`: 失敗

**処理:**
1. `rte_eal_remote_launch()` で関数をlcoreにディスパッチ
2. `CURRENT_LCORE_ID` を次のlcoreに更新

## 使用例
```rust
// ワーカー関数
pub extern "C" fn worker(args: *mut c_void) -> i32 {
    let args = unsafe { &mut *(args as *mut WorkerArgs) };
    loop {
        // 処理
    }
}

// スポーン
let mut args = WorkerArgs { ... };
spawn(worker, &mut args as *mut WorkerArgs as *mut c_void);
```

## 注意点
- 関数は`extern "C"`で`i32`を返す必要がある
- lcoreは有限 (EAL引数`-l`で指定)
- スポーン後、argsのライフタイムに注意

## 関連ファイル
- `helper/dpdk.rs`: `thread_init()`呼び出し
- `controller.rs`: ワーカースポーン
