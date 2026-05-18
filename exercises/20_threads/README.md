# Threads

In most current operating systems, an executed program's code is run in a process, and the operating system manages multiple processes at once.
Within your program, you can also have independent parts that run simultaneously. The features that run these independent parts are called threads.

## Further information

- [Dining Philosophers example](https://doc.rust-lang.org/1.4.0/book/dining-philosophers.html)
- [Using Threads to Run Code Simultaneously](https://doc.rust-lang.org/book/ch16-01-threads.html)
- [Using Message Passing to Transfer Data Between Threads](https://doc.rust-lang.org/book/ch16-02-message-passing.html)

---

## 学习笔记

### threads1.rs — 线程创建与 JoinHandle

`thread::spawn` 的签名：
```rust
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
```

- 传入的闭包 `move || { ... }` 会在新线程中执行
- `move` 关键字将外部变量的所有权**转移**到闭包内（此处是 `i`）
- 返回值是 `JoinHandle<T>`，`T` 是闭包的返回值类型

**收集线程结果：**
```rust
let mut results = Vec::new();
for handle in handles {
    results.push(handle.join().unwrap());
}
```

- `join()` — 阻塞当前线程，等待子线程结束，返回 `Result<T>`
- `unwrap()` — 如果线程成功结束就取出值，如果线程 panic 了则当前线程也会 panic

| 概念 | 说明 |
|---|---|
| `thread::spawn` | 创建一个新线程，OS 调度执行 |
| `JoinHandle` | 线程的"遥控器"，可等待结束、取结果 |
| `join()` | 阻塞等待，返回值是 `Result<T>` |
| `move` 闭包 | 强制闭包获取用到的变量的所有权 |

---

### threads2.rs — 共享可变状态：`Arc<Mutex<T>>`

多线程环境下要**同时**满足两个需求：

| 需求 | 用什么 | 解决什么问题 |
|---|---|---|
| 多个线程共享同一份数据 | `Arc<T>` | 多所有权（引用计数，线程安全版 `Rc`） |
| 修改共享数据 | `Mutex<T>` | 内部可变性 + 互斥访问 |

**核心代码：**
```rust
use std::sync::{Arc, Mutex};

// 创建：两层包装
let status = Arc::new(Mutex::new(JobStatus { jobs_done: 0 }));

// 跨线程传递：clone Arc（引用计数+1）
let status_shared = Arc::clone(&status);

// 子线程中修改：lock() 拿到守卫 → 修改 → 守卫离开作用域自动解锁
status_shared.lock().unwrap().jobs_done += 1;

// 主线程中读取：
println!("Jobs done: {}", status.lock().unwrap().jobs_done);
```

**`lock()` 返回什么？**
```rust
pub fn lock(&self) -> LockResult<MutexGuard<'_, T>>
```
- `lock()` 返回 `Result<MutexGuard<T>>`，如果其他线程持有锁时 panic（锁被"毒化"），返回 `Err`
- `unwrap()` 拿到 `MutexGuard`，它实现了 `DerefMut`，可以当 `&mut T` 用
- `MutexGuard` 的 `Drop` 自动释放锁

**类比理解：**
```
Arc   = 一本公共图书的"借书证复印机"（多人都能拿着凭证去借书）
Mutex = 图书馆柜台"只允许一个人借阅"的规则（互斥锁）
lock()= 你去柜台排队拿到书（获取守卫）
离开作用域 = 你把书还回去（Drop 释放锁）
```

---

### threads3.rs — 消息传递：`mpsc::channel`

`mpsc` = **m**ulti-**p**roducer, **s**ingle-**c**onsumer（多生产者、单消费者）

**核心代码：**
```rust
use std::sync::mpsc;

// 创建通道：得到一个发送端和一个接收端
let (tx, rx) = mpsc::channel();

// tx 可以 clone，每个 clone 都可以独立发送数据
let tx1 = tx.clone();

thread::spawn(move || { tx1.send(42).unwrap(); });
thread::spawn(move || { tx.send(100).unwrap(); });

// 接收端迭代：会阻塞等待直到所有发送端都 drop
for value in rx {
    println!("{}", value);  // 42 和 100（顺序不定）
}
```

**本题关键点：**

| 操作 | 为什么 |
|---|---|
| `tx.clone()` | `Sender` 只能有一个线程拥有，clone 后两个线程各持一个发送端 |
| 解构 `Queue` | `first_half` 和 `second_half` 分别给两个线程，避免所有权冲突 |

**`for value in rx` 的终止条件：**
- `rx` 会一直等待新数据
- 当**所有** `Sender` 都被 `drop` 时，通道关闭
- 通道关闭后，`rx` 迭代自动结束，不会死循环

**与 `Arc<Mutex<T>>` 的对比：**

| | `mpsc::channel` | `Arc<Mutex<T>>` |
|---|---|---|
| 理念 | 不要共享内存，而是传递消息 | 共享内存，靠锁保护 |
| 数据流向 | 单向：多个S → 1个R | 任意线程读写同一块数据 |
| 同步开销 | 无锁（通道内部有缓冲区） | 有锁竞争 |
| 适合场景 | 生产者-消费者、流水线 | 需要多线程读写同一份状态 |
| Rust 名言 | "Do not communicate by sharing memory" | "Share memory by communicating" |

---

### `thread::spawn(move || {})` 详解

**结构拆解：**

| 部分 | 含义 |
|---|---|
| `thread::spawn(...)` | 创建新线程，传入一个闭包作为线程的入口函数 |
| `\|\|` | 闭包的参数列表，空 = 无参数 |
| `{ ... }` | 闭包体，线程执行的代码 |
| `move` | 强制闭包**获取**外部变量的所有权，而非借用 |

**有无 `move` 的对比：**

```rust
let s = String::from("hello");

// ❌ 不带 move：闭包尝试借用 s
//    编译器报错：s 的生命周期可能不够长
thread::spawn(|| {
    println!("{s}");
});

// ✅ 带 move：s 的所有权移入闭包
//    此后主线程不能再使用 s
thread::spawn(move || {
    println!("{s}");
});
// println!("{s}");  // 编译错误：s 已被 move 走
```

**`Copy` 类型下 `move` 的行为：**
```rust
for i in 0..10 {
    // i 是 i32（Copy 类型），move 相当于复制一份
    thread::spawn(move || {
        println!("{i}");  // 每个线程持有自己的 i 副本
    });
}
```

**何时必须加 `move`：**
- 闭包要传给其他线程（`Send + 'static` 约束）
- 闭包可能活得比当前作用域久
- 闭包需要持有而非借用外部数据

---

### Rust 闭包（Closure）基础

**定义：** 闭包是**可以捕获外部环境变量**的匿名函数。

**语法：**
```rust
|参数1, 参数2| { 函数体 }
|a, b| a + b              // 单表达式，省略花括号
|| println!("hi")          // 无参数
```

**三种捕获方式（对应三个 trait）：**

| 捕获方式 | Trait | 说明 | 代码示例 |
|---|---|---|---|
| 不可变借用 | `Fn` | 只读外部变量，可多次调用 | `\| \| println!("{x}")` |
| 可变借用 | `FnMut` | 修改外部变量，可多次调用 | `\| \| { x += 1; }` |
| 拿走所有权 | `FnOnce` | `move` 进来，只能调一次 | `move \|\| drop(s)` |

**函数 vs 闭包：**

| | 函数 `fn add(){}` | 闭包 `\|a\| a+1` |
|---|---|---|
| 名字 | 必须有 | 匿名（可绑定到变量） |
| 类型标注 | 必须写 | 编译器推断，可省略 |
| 捕获环境 | ❌ 不能 | ✅ 能（核心区别） |
| 位置 | 顶层定义 | 可以就地写在表达式里 |

**常见用法：**
```rust
// 1. 迭代器适配器
vec![1, 2, 3].iter().map(|x| x + 1).collect::<Vec<_>>();

// 2. 线程入口
thread::spawn(move || { /* 线程代码 */ });

// 3. sort 自定义比较
v.sort_by(|a, b| a.len().cmp(&b.len()));
```
