# Smart Pointers

In Rust, smart pointers are variables that contain an address in memory and reference some other data, but they also have additional metadata and capabilities.
Smart pointers in Rust often own the data they point to, while references only borrow data.

## Further Information

- [Smart Pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)
- [Using Box to Point to Data on the Heap](https://doc.rust-lang.org/book/ch15-01-box.html)
- [Rc\<T\>, the Reference Counted Smart Pointer](https://doc.rust-lang.org/book/ch15-04-rc.html)
- [Shared-State Concurrency](https://doc.rust-lang.org/book/ch16-03-shared-state.html)
- [Cow Documentation](https://doc.rust-lang.org/std/borrow/enum.Cow.html)

---

## 学习笔记

### 1. `Box<T>` — 把数据放到堆上

**为什么需要它？** Rust 编译时必须知道每个类型的大小。递归类型（类型包含自身）会让编译器无法计算大小，`Box<T>` 将数据放堆上，栈上只存固定大小的指针。

```rust
// ❌ 编译错误：recursive type has infinite size
enum List {
    Cons(i32, List),
    Nil,
}

// ✅ 用 Box 解决
enum List {
    Cons(i32, Box<List>),
    Nil,
}

let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
```

**其他用途**：传递大型数据避免栈拷贝；`Box<dyn Trait>` 做 trait object。

---

### 2. `Rc<T>` — 引用计数，多个所有者

**核心问题**：Rust 所有权规则规定每个值只有一个所有者，但有时需要多处共享同一份数据。

```rust
use std::rc::Rc;

let sun = Rc::new(Sun);        // 引用计数 = 1
let m = Rc::clone(&sun);       // 引用计数 = 2 （不深拷贝，只加计数）
let v = Rc::clone(&sun);       // 引用计数 = 3
// 计数归零时数据才被释放
```

| | `&T`（引用） | `Rc<T>` |
|---|---|---|
| 所有权 | 借用，不拥有 | 共享所有权 |
| 生存期 | 受生命周期约束 | 运行时动态管理 |
| 可变性 | 可有 `&mut T` | 只能不可变访问 |
| 线程 | ✅ | ❌ 单线程 |

---

### 3. `Arc<T>` — 线程安全的 Rc

`Arc` = **A**tomic **R**eference **C**ounting。与 `Rc` 功能相同，但使用原子操作计数，线程安全。

```rust
use std::sync::Arc;
use std::thread;

let data = Arc::new(vec![1, 2, 3]);
let mut handles = vec![];

for _ in 0..3 {
    let d = Arc::clone(&data);
    handles.push(thread::spawn(move || {
        println!("{:?}", d);
    }));
}

for h in handles { h.join().unwrap(); }
```

| | `Rc<T>` | `Arc<T>` |
|---|---|---|
| 线程安全 | ❌ | ✅ |
| 计数方式 | 普通加减 | 原子操作（略慢） |
| 场景 | 单线程共享 | 多线程共享 |

**原则**：单线程用 `Rc`，多线程用 `Arc`。

---

### 4. `Cow<T>` — 写时克隆（Clone on Write）

**核心思想**：先用别人的数据，实在要改时再自己复制一份。

```rust
use std::borrow::Cow;

fn abs_all(input: &mut Cow<[i32]>) {
    for i in 0..input.len() {
        if input[i] < 0 {
            input.to_mut()[i] = -input[i];  // 需要修改时才克隆！
        }
    }
}

let v = vec![1, 2, 3];
let mut cow = Cow::from(&v);
abs_all(&mut cow);
// → Cow::Borrowed，没有分配新内存（全是正数，不用改）

let v = vec![-1, 2, 3];
let mut cow = Cow::from(&v);
abs_all(&mut cow);
// → Cow::Owned，自动克隆了一份（有负数需要改）
```

**典型场景**：函数接收 `&str`，只在需要修改时才转为 `String`，避免不必要的内存分配。

---

### 总结对比

| 智能指针 | 一句话 | 关键区别 |
|---------|--------|---------|
| `Box<T>` | 数据放堆上 | 唯一所有者，最简单 |
| `Rc<T>` | 多人共享 | 引用计数，单线程 |
| `Arc<T>` | 跨线程共享 | Rc + 原子操作 |
| `Cow<T>` | 能省就省 | 惰性克隆，不修改就不拷贝 |

---

### 补充：`dyn Trait` 与 `Box<dyn Trait>`

`dyn` = **dynamic dispatch（动态分发）**，表示"一个实现了某 trait 的任意类型"。

#### 静态分发 vs 动态分发

```rust
trait Animal {
    fn speak(&self);
}
struct Dog;
impl Animal for Dog { fn speak(&self) { println!("汪汪"); } }
struct Cat;
impl Animal for Cat { fn speak(&self) { println!("喵喵"); } }

// 静态分发（编译时确定，为每种类型生成一份代码）
fn static_call<T: Animal>(animal: &T) {
    animal.speak();
}

// 动态分发（运行时通过虚表 vtable 查找）
fn dynamic_call(animal: &dyn Animal) {
    animal.speak();
}
```

#### `dyn` 为什么需要指针？

`dyn Animal` 代表任意实现了 `Animal` 的类型，不同具体类型大小不同，编译器无法确定其大小，所以必须通过指针间接访问：

```rust
&dyn Animal           // 引用
Box<dyn Animal>       // 智能指针（最常用）
Arc<dyn Animal>       // 多线程共享
```

#### 典型用途：集合中放不同类型

```rust
let animals: Vec<Box<dyn Animal>> = vec![
    Box::new(Dog),
    Box::new(Cat),
    Box::new(Dog),
];

for animal in &animals {
    animal.speak();  // 汪汪 喵喵 汪汪
}
```

| | 静态分发 `<T: Trait>` | 动态分发 `dyn Trait` |
|---|---|---|
| 调用方式 | 编译时跳转（单态化） | 运行时虚表查找 |
| 性能 | 更快（无运行时开销） | 有轻微虚表开销 |
| 二进制大小 | 每种类型生成一份代码（更大） | 只生成一份代码 |
| 集合异构 | 不能（Vec 只能放同类型） | 能（Vec 可放不同类型） |
| 何时使用 | 类型已知、性能敏感 | 需要类型擦除、异构集合 |

---

### 补充：`drop` 函数

`drop` 用于**手动提前释放一个值**，不等离开作用域就销毁它。

```rust
let s = String::from("hello");
drop(s);           // 提前释放，s 被移动走
// s.push_str(" world");  // ❌ 编译错误：s 已失效
```

与 `Rc` 配合，手动减少引用计数：

```rust
let neptune = Planet::Neptune(Rc::clone(&sun));  // 计数 +1
drop(neptune);  // 计数 -1，不等作用域结束就释放
```

本质：`drop(x)` 是一个标准库函数，内部调用 `x` 的 `Drop::drop` 方法（析构函数），执行清理（释放内存、关闭文件、减少 `Rc` 计数等）。它不是关键字。

---

### 补充：`Rc::strong_count`

返回 `Rc` 的当前强引用计数（即多少个所有者共享这份数据）。

```rust
let sun = Rc::new(Sun);
Rc::strong_count(&sun);  // → 1

let m = Planet::Mercury(Rc::clone(&sun));
Rc::strong_count(&sun);  // → 2

let v = Planet::Venus(Rc::clone(&sun));
Rc::strong_count(&sun);  // → 3
```

每次 `Rc::clone` → 计数 +1；`drop` 或离开作用域 → 计数 -1；计数归零 → 数据释放。

附：还有弱引用 `Rc::weak_count`，通过 `Rc::downgrade` 创建，不阻止数据释放，需升级为强引用才能访问。

---

### 补充：`Rc::new` vs `Rc::clone`

| | `Rc::new(value)` | `Rc::clone(&rc)` |
|---|---|---|
| 作用 | 新建 Rc，堆上分配数据 | 共享已有数据 |
| 计数 | 初始化为 1 | 计数 +1 |
| 开销 | 分配内存 | 仅整数加一（极轻量） |
| 返回值 | 新的 Rc 句柄 | 指向同一数据的新句柄 |

```rust
let sun = Rc::new(Sun);     // 堆上分配 Sun，计数 = 1
let m = Rc::clone(&sun);    // 不拷贝数据，计数变 2
// sun 和 m 指向堆上同一个 Sun

// 也可以写成 sun.clone()，但 Rc::clone(&sun) 更明确表达"增加引用计数"
```

> 注意：`Rc::clone` 与普通 `.clone()` 不同，普通 clone 是深拷贝数据，`Rc::clone` 只是计数值 +1。

---

### 补充：`Rc::new` 的参数必须是值（move 语义）

```rust
let s = Sun;
let sun = Rc::new(s);     // s 被移入 Rc，所有权转移
// s 不再可用

let sun = Rc::new(&s);    // ❌ 不能传引用，Rc 需要拥有数据本身
```

`Rc::new`、`Box::new`、`Vec::new` 都一样——参数被 move 进去。

---

### 缩写含义

| 缩写 | 全称 | 含义 |
|------|------|------|
| `Rc` | **R**eference **C**ounted | 引用计数 |
| `Arc` | **A**tomic **R**eference **C**ounted | 原子引用计数 |
| `Cow` | **C**lone **o**n **W**rite | 写时克隆 |
| `Box` | 不是缩写 | 就是"盒子"，数据装进堆上的盒子 |

---

### 补充：Cow 与 Deref

`Cow` 实现了 `Deref<Target = [i32]>`，所以 `&mut Cow<[i32]>` 能自动解引用为 `[i32]`：

```rust
fn abs_all(input: &mut Cow<[i32]>) {
    input.len();      // 自动解引用 → (*input).len()，即 [i32] 的 len
    input[ind];       // 自动解引用 → (*input)[ind]，即 [i32] 的索引
    // to_mut() 不能自动调用，必须显式写
    input.to_mut()[ind] = -value;
}
```

Deref 让 Cow 在读时零开销表现得像内部数据，写时再通过 `to_mut()` 显式触发克隆。

### 补充：为什么 Cow 的 mut 不能直接修改内部

`&mut Cow<[i32]>` 的 `mut` 只能**替换整个 Cow 的值**，不能直接修改内部的 `[i32]`：

```rust
let mut cow = Cow::Borrowed(&[1, 2, 3][..]);
// 通过 Deref 拿到的是 &[i32]（不可变），不是 &mut [i32]
cow[0];            // ✅ 读
// cow[0] = 5;     // ❌ Deref 只给不可变引用

cow.to_mut()[0] = 5;  // ✅ to_mut() 返回 &mut [i32]
// 如果 Cow 是 Borrowed，这行会先自动克隆，再修改
```

`Cow` 把"可变访问内部数据"藏在 `to_mut()` 后面，这样它才能在需要时自动决定是否克隆。`&mut Cow` 的 `mut` 和 `&mut [i32]` 的 `mut` 不是一回事。
