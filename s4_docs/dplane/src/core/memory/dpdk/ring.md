# dplane/src/core/memory/dpdk/ring.rs

## 概要
DPDKのrte_ringを使用したロックフリーリングバッファ。マルチプロデューサー/マルチコンシューマー (MPMC) 対応。

## 構造体

### Ring
```rust
pub struct Ring {
    ring: *mut rte_ring,
}

unsafe impl Send for Ring {}
```

### RingBuf<T>
オブジェクトプール。メモリプール機能を提供。

```rust
pub struct RingBuf<T> {
    phantom: PhantomData<T>,
    mempool: *mut rte_mempool,
}
```

## Ring関数

### Ring::new(len: usize) -> Self
新しいリングバッファを作成。

**フラグ:**
- `RING_F_MP_RTS_ENQ`: RTSモードマルチプロデューサー
- `RING_F_MC_RTS_DEQ`: RTSモードマルチコンシューマー

### ring.enqueue<T>(&self, obj: &mut T) -> i32
オブジェクトをエンキュー。

**戻り値:**
- `0`: 成功
- `< 0`: 失敗 (リング満杯)

### ring.dequeue<T>(&self, obj: &mut &mut T) -> i32
オブジェクトをデキュー。

### ring.enqueue_burst<T>(&self, objs: &&mut T, len: usize) -> usize
バルクエンキュー。

### ring.dequeue_burst<T>(&self, objs: &Array<&mut T>, len: usize) -> usize
バルクデキュー。

### ring.dequeue_burst_resume<T>(..., pos: usize, ...) -> usize
オフセット位置からバルクデキュー。

## RingBuf関数

### RingBuf::new(len: usize) -> Self
オブジェクトプールを作成。

### ringbuf.malloc<'a>(&'a self) -> &'a mut T
オブジェクトを取得。

### ringbuf.free(&self, obj: &mut T)
オブジェクトを返却。

### ringbuf.malloc_bulk(&self, obj: &mut [&mut T], len: usize)
バルク取得。

### ringbuf.free_bulk(&self, obj: &[&mut T], len: usize)
バルク返却。

## マクロ

### init_ringbuf_element!
プール全要素を初期化。

```rust
init_ringbuf_element!(ringbuf, Pkt, {
    owner_ring => &mut ringbuf,
    field => value,
});
```

### malloc_ringbuf_all_element!
全要素を一括取得。

### free_ringbuf_all_element!
全要素を一括返却。

## 使用例

### Ring
```rust
let ring = Ring::new(65536);

// 送信側
ring.enqueue(&mut obj);

// 受信側
let buf = Array::<&mut Obj>::new(64);
let count = ring.dequeue_burst::<Obj>(&buf, 64);
for i in 0..count {
    process(buf.get(i));
}
```

### RingBuf
```rust
let pool = RingBuf::<Pkt>::new(8192);

// 取得
let pkt = pool.malloc();
pkt.data = ...;

// 返却
pool.free(pkt);
```

## パフォーマンス特性
- ロックフリー設計
- キャッシュライン整列
- バースト操作でスループット向上

## 関連ファイル
- `array.rs`: バッファ配列
- `worker/rx.rs`: RX側でRingBuf使用
- `worker/pipeline.rs`: Pipeline側でRing使用
