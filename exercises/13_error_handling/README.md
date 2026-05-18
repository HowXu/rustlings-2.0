# Error handling

Most errors aren't serious enough to require the program to stop entirely.
Sometimes, when a function fails, it's for a reason that you can easily interpret and respond to.
For example, if you try to open a file and that operation fails because the file doesn't exist, you might want to create the file instead of terminating the process.

## Further information

- [Error Handling](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
- [Generics](https://doc.rust-lang.org/book/ch10-01-syntax.html)
- [Result](https://doc.rust-lang.org/rust-by-example/error/result.html)
- [Boxing errors](https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/boxing_errors.html)


讲的是Result类吧

---

## errors2 — `?` 运算符

### 问题
`parse()` 返回 `Result<i32, ParseIntError>`，需要用 `?` 传播错误，而非 `unwrap()` 导致 panic。

### 关键点
`?` 运算符的本质是 `match` + `return Err` 的语法糖：

```rust
// 这两种写法完全等价：
let qty = item_quantity.parse::<i32>()?;

let qty = match item_quantity.parse::<i32>() {
    Ok(n) => n,
    Err(e) => return Err(e.into()),
};
```

### 使用条件
`?` 只能在返回 `Result` 或 `Option` 的函数中使用，且错误类型必须兼容（能 `From::from` 转换）。本题中 `parse` 的错误类型 `ParseIntError` 恰好与函数返回的错误类型一致，无需额外转换。

---

## errors6 — `map_err` 错误类型转换 + `?` 传播

### 自定义错误枚举
```rust
enum ParsePosNonzeroError {
    Creation(CreationError),   // 来自 new() 的业务错误
    ParseInt(ParseIntError),   // 来自 parse() 的解析错误
}
```

### `map_err` 的本质
只转换 `Err` 分支，`Ok` 原封不动：

```rust
// map_err 伪代码
fn map_err<T, E, F>(self, op: impl FnOnce(E) -> F) -> Result<T, F> {
    match self {
        Ok(val) => Ok(val),
        Err(e)  => Err(op(e)),
    }
}
```

### 错误流转全过程
```
s.parse::<i64>()
    → Result<i64, ParseIntError>          // 错误类型不匹配
    .map_err(PNPError::ParseInt)          // 转换: ParseIntError → PNPError
    → Result<i64, PNPError>
    ?                                      // 取出值，或提前返回 Err
    → i64

Self::new(x)
    → Result<Self, CreationError>         // 错误类型不匹配
    .map_err(PNPError::from_creation)     // 转换: CreationError → PNPError
    → Result<Self, PNPError>              // ← 恰是函数签名，直接作为返回值
```

### 为什么一处用 `?` 一处不用？
| 位置 | 返回类型需求 | 操作 |
|------|-------------|------|
| `parse` 之后 | 需要 `i64` 存到变量 | 用 `?` 取出值 |
| `new` 之后 | 需要 `Result<Self, PNPError>` | 直接作为函数返回值，不用 `?` |

### 枚举变体直接当函数用
```rust
// PNPError::ParseInt 本身就是一个 fn(ParseIntError) -> PNPError
// 所以可以直接传给 map_err：
s.parse::<i64>().map_err(ParsePosNonzeroError::ParseInt)

// 等价于闭包写法：
s.parse::<i64>().map_err(|e| ParsePosNonzeroError::ParseInt(e))
```

---

## ⚠️ 重要补充：`JoinHandle::join()` 也返回 `Result`

在 threads1 练习中，`handle.join().unwrap()` 用到了 `Result`：

```rust
// JoinHandle::join() 的签名
pub fn join(self) -> Result<T, Box<dyn Any + Send + 'static>>
```

### 为什么返回 `Result`？

线程可能 **panic**。如果子线程触发了 panic，`join()` 会捕获这个 panic 信息，包装在 `Err` 变体中返回。

- `Ok(T)` — 线程正常结束，内含返回值
- `Err(Box<dyn Any>)` — 线程 panic 了，内含 panic 时传入的值（字符串等）

### 对比：I/O 错误 vs 线程 panic

| 场景 | 错误类型 | 含义 |
|---|---|---|
| `File::open` 失败 | `Err(std::io::Error)` | **可预期的**运行失败 |
| `parse` 失败 | `Err(ParseIntError)` | 数据格式错误 |
| 子线程 panic | `Err(Box<dyn Any + Send>)` | **非预期的**逻辑错误，类似程序"崩了" |

### `unwrap()` 的含义

```rust
handle.join().unwrap()
//               ^^^^^^
// 假设线程不会 panic，直接取结果
// 如果线程真的 panic 了 → 当前线程也跟着 panic
```

`unwrap()` 在这里等于说："我信任子线程不会 panic，如果它 panic 了，那我也崩掉算了。" 这是编写练习时的简便写法，生产代码中可能需要更细致的处理
```