# Rustlings 练习笔记 (Exercise 0-23) ✅ 全部完成

> 提取自 `exercises/00_intro` ~ `exercises/23_conversions` 的学习笔记。
> 分为三篇 + 专题合集：上篇(基础)、中篇(核心)、下篇(进阶)。

---

# 上篇：基础语法 (0-8)

## 0. Intro（入门）

Rust 使用 `print!` 和 `println!` 宏向控制台输出文本。宏调用尾部必须带 `!`。

---

## 1. Variables（变量）

### 基本规则

- **变量默认不可变**。一旦值绑定到名称，就不能更改。
- 加 `mut` 可使绑定变为可变：`let mut x = 3;`
- 参数默认也不可变：`fn f(x: i32)` 中不能 `x = 5`

### 遮蔽（Shadowing）

用 `let` 重新声明同名变量来遮蔽旧变量，类型可以自由切换：

```rust
let number = "T-H-R-E-E";
let number = 3;   // 遮蔽，类型从 &str 变为 i32
```

### 常量（const）

`const NUMBER: i32 = 3;` — 命名惯例全大写，必须标注类型。

### 三层独立性（Values / Bindings / References）

```
值的可变性  → 值有 &mut 时就能改，不管 let 是否加 mut
绑定的可变性 → let mut 才能重新赋值这个变量名
引用的可变性 → &mut 给你"可以改"的令牌，& 只给"看"的令牌
```

| | 所有权 | 只读借用 | 读写借用 |
|---|---|---|---|
| 绑定 | `let a = v` | `let r = &v` | `let rm = &mut v` |
| 权限 | 读+释放 | 只读 | 读+写 |
| 写法 | 不需 mut | 不需 mut | 需要 mut（在 v） |

### 函数参数上的 mut vs &mut

```rust
fn foo(mut x: i32) {   // mut 只允许在函数内修改局部副本
    x = x + 1;
}
// 调用方不受影响，x 是复制进去的

fn bar(x: &mut i32) {  // &mut 借来改原值
    *x += 1;
}
// 调用方原值被修改
```

| | `fn f(mut x: T)` | `fn f(x: &mut T)` |
|---|---|---|
| 改的是 | 函数内的局部副本 | 调用方原来的值 |
| 调用方写法 | `f(val)` | `f(&mut val)` |
| 调用方变量 | 不需要 mut | 必须是 `let mut` |

### Copy 语义

| 类型 | 赋值行为 | 原变量 |
|---|---|---|
| `Copy` 类型（`i32`, `bool`, `f64` 等） | 自动复制 | 仍可用 |
| 非 `Copy` 类型（`String`, `Vec` 等） | 所有权转移 | 失效 |

---

## 2. Functions（函数）

### 函数签名与返回

- 参数必须标注类型：`fn call_me(num: i64)`
- 返回值用 `->` 标注：`fn square(num: i32) -> i32`
- 函数签名不加 `mut`，因为 `mut` 不是类型的一部分

### 语句 vs 表达式

- 带 `;` 是语句（statement），不返回值
- 不带 `;` 是表达式（expression），会作为返回值
- 函数体最后一个表达式就是返回值（无需 `return` 关键字）

```rust
fn square(num: i32) -> i32 {
    num * num   // 无 ; → 表达式，作为返回值
}
```

---

## 3. If（条件控制）

### `if` 是表达式

直接用于赋值（C/Java 需要三元运算符 `?:`）：

```rust
let identifier = if animal == "crab" {
    1
} else if animal == "gopher" {
    2
} else {
    0
};  // 注意这里需要 ;
```

### 关键规则

- 所有分支必须返回相同类型
- 只有 `if` 没 `else` 时，隐含返回 `()`
- 条件不需要括号，但必须是 `bool`（不支持 `if 1` 隐式转换）

> 更多控制流内容见 **[专题一：Rust 控制流全解](#专题一rust-控制流全解)**

---

## 4. Primitive Types（原始类型）

### bool

```rust
let is_morning: bool = true;
```

### char

使用**单引号**：`let c = 'A';`，支持 `is_alphabetic()`、`is_numeric()` 等方法。

### 数组（Array）`[T; N]`

栈上固定大小，`N` 是类型的一部分：

```rust
let a: [i32; 100] = [5; 100];  // 100个元素，全部初始化为5
```

### 切片（Slice）`[T]`

动态大小类型（DST），必须藏在指针后面：`&[T]`

```rust
let nice_slice = &a[1..4];  // 左闭右开区间 [1, 4)
```

| 特性 | 数组 `[T; N]` | 切片 `[T]` |
|------|--------------|-----------|
| 大小 | 编译时确定 | 运行时确定（DST） |
| 可否直接持值 | 可以，栈上分配 | 不能，只能通过 `&[T]` |
| 类型区分 | `[i32; 3]` ≠ `[i32; 5]` | 不同长度同属 `[i32]` |
| 内存 | N × sizeof(T) | 胖指针 (ptr + len, 16字节) |

`&[T]` 是胖指针：占 2 个 usize（指针 + 长度）。数组自动 Deref 为切片引用。

### 元组（Tuple）

```rust
let cat = ("Furry McFurson", 3.5);

// 解构
let (name, age) = cat;

// 索引访问（从 0 开始）
let second = numbers.1;

// 单元素元组需要尾部逗号
let single = (42,);
```

---

## 5. Vecs（向量）

### 基本概念

- **数组**：栈上分配，大小编译期确定，不可增缩
- **向量（Vec）**：堆上分配，可动态增缩

### 创建

```rust
let v = vec![10, 20, 30, 40]; // 宏创建
let v = a.to_vec();           // 从数组转换
let v: Vec<i32> = Vec::new(); // 空向量，需标注类型
```

### 可变性

`Vec` 默认不能 `push` / 删除元素，必须声明为 `mut`：

```rust
let mut output = Vec::new();
output.push(42);
```

### vec! 宏注意事项

`vec!` 里放的是**具体的值**，不是类型名：

```rust
// ❌ 这是把类型名当元素
// let mut result = vec![&str];

// ✅ 正确写法
let mut result: Vec<&str> = vec![];
let mut result = vec!["hello", "world"];
```

---

## 6. Move Semantics（移动语义）

### 核心规则

**一个值同时只能有一个所有者。** 赋值、传参、返回值时，所有权转移，原变量失效。

```rust
let s1 = String::from("hello");
let s2 = s1;              // 所有权从 s1 移动到 s2
// println!("{}", s1);   // 编译错误！s1 已失效
```

### 移动 vs 拷贝

| 类型 | 行为 | 原变量 |
|---|---|---|
| `i32`, `bool` 等（Copy trait） | 自动复制 | 仍可用 |
| `String`, `Vec` 等（堆数据） | 移动 | 失效 |

需要保留原变量时，显式使用 `.clone()` 深拷贝。

### `let mut vec = vec;` 发生了什么？

移动语义，不是深拷贝也不是浅拷贝：
- 底层仅复制栈上的 3 个字段（指针、长度、容量）
- 堆上的数据没有复制
- 原绑定被编译器标记为失效，防止 double-free

### mut 在参数上的含义

`mut` 是**绑定的属性**，不是类型的一部分。即使传入的是不可变绑定，所有权转移后函数内部可以用 `mut` 声明新的可变绑定来修改它。

### 可变借用规则

**同一时刻只能有一个 `&mut` 引用**（防止数据竞争）：

```rust
let mut x = Vec::new();
let z = &mut x;  // 拿走可变借用
z.push(42);      // z 的最后一次使用，借用结束
let y = &mut x;  // 可以再拿了
```

### `.` 操作符的自动借用

```rust
v.push(4)   // → 方法调用自动取引用 → &mut v → 传给 push(&mut self, ...)
```

`.` 会自动取 `&` 或 `&mut`，但不能无中生有——`&mut` 只能从 `mut` 绑定生成。

### 字段访问不会移走所有权

```rust
struct Rectangle { width: i32, height: i32 }
let rect = Rectangle::new(10, 20); // rect 拥有整个结构体
assert_eq!(rect.width, 10);        // 仅读取字段，rect 所有权不变
```

| 操作 | 所有权变化 |
|------|-----------|
| `rect.width` 字段访问 | 不变（读取） |
| `&rect` 借用 | 不变（共享借用） |
| `let rect2 = rect;` 赋值 | 转移，rect 失效 |
| `fn consume(r: Rectangle)` 传参 | 转移，实参失效 |

---

## 7. Structs（结构体）

### 三种结构体

```rust
// 经典结构体（字段名访问）
struct ColorRegularStruct { red: u8, green: u8, blue: u8 }

// 元组结构体（下标访问 .0, .1, .2）
struct ColorTupleStruct(u8, u8, u8);

// 单元结构体（无字段，值就是名字本身）
#[derive(Debug)]
struct UnitStruct;
```

### 实例化

```rust
// 经典 — 字段名:值
let green = ColorRegularStruct { red: 0, green: 255, blue: 0 };

// 元组 — 按位置传值
let green = ColorTupleStruct(0, 255, 0);

// 单元 — 值就是名字
let unit = UnitStruct;
```

### 结构体更新语法

```rust
let your_order = Order {
    name: String::from("Hacker in Rust"),
    count: 1,
    ..order_template   // 其余字段继承
};
```

### 方法（impl）

```rust
impl Package {
    // 关联函数（静态方法 / 工厂）：无 &self，用 Self 返回类型
    fn new(sender_country: String, weight_in_grams: u32) -> Self {
        Self { sender_country, weight_in_grams }  // 字段简写
    }

    // 实例方法：&self 借用实例
    fn is_international(&self) -> bool { ... }

    fn get_fees(&self, cents_per_gram: u32) -> u32 { ... }
}
```

- **无 `self` 参数**（如 `fn new() -> Self`）是关联函数，调用用 `Package::new(...)`
- **`&self` 参数**是实例方法，调用用 `package.method()`，`.` 自动取引用
- **`Self`** 指代 `impl` 所在的类型名
- **优先用 `&self`**：借用不会触发所有权转移

### `#[derive(Debug)]`

自动实现 `Debug` trait，允许 `{:?}` / `{:#?}` 打印。

---

## 8. Enums（枚举）

### 基本枚举

```rust
#[derive(Debug)]
enum Message { Resize, Move, Echo, ChangeColor, Quit }
```

### 带数据的变体（变体即构造函数）

```rust
enum Message {
    Resize { width: u64, height: u64 },  // 命名字段
    Move(Point),                          // 含一个 Point
    Echo(String),                         // 含一个 String
    ChangeColor(u8, u8, u8),             // 含三个 u8
    Quit,                                 // 无数据
}
```

这些变体是**构造函数**，不是函数。每次写 `Message::Move(Point { x: 10, y: 15 })` 就是在创建值。同一个 `Vec<Message>` 可以装下各种形态的消息，然后用 `match` 分流处理——这是 Rust 版的**代数数据类型（ADT）/ 多态**。

### match 模式匹配

```rust
fn process(&mut self, message: Message) {
    match message {
        Message::Resize { width, height } => self.resize(width, height),
        Message::Move(p)                   => self.move_position(p),
        Message::Echo(s)                   => self.echo(s),
        Message::ChangeColor(r, g, b)      => self.change_color(r, g, b),
        Message::Quit                      => self.quit(),
    }
}
```

- **必须穷尽**（exhaustive）：漏掉任何变体编译器报错
- **模式解构**：`Message::Resize { width, height }` 直接取出内部字段

### 格式化输出

| 格式 | 用途 | 所需 trait | 获取方式 |
|---|---|---|---|
| `{}` | 用户友好输出 | `Display` | 手动实现 |
| `{:?}` | 调试输出 | `Debug` | `#[derive(Debug)]` |
| `{:#?}` | 美化调试输出 | `Debug` | 同上 |

### Match Guard（守卫）

`模式 if 条件`，模式匹配通过后再检查条件：

```rust
match point {
    (x, y) if x == y         => println!("在对角线上"),
    (x, y) if x > 5 && y > 5 => println!("都在右上角"),
    _                        => println!("其他位置"),
}

// 枚举解构 + guard
match shape {
    Shape::Circle(r) if r > 100.0         => println!("大圆"),
    Shape::Rectangle(w, h) if w * h > 50.0 => println!("大面积矩形"),
    _ => println!("其他"),
}
```

| 特性 | match guard | if/else |
|------|-------------|---------|
| 模式解构 | ✅ 内置 | ❌ 需先 `let` 解构 |
| 穷尽性检查 | ✅ 编译器强制 | ❌ 可能遗漏 |

> 更多模式匹配内容见 **[专题二：let 模式解构详解](#专题二let-模式解构详解)**

---

## 速查表 (0-8)

| 概念 | 语法 | 要点 |
|---|---|---|
| 变量声明 | `let x = 5;` | 默认不可变 |
| 可变变量 | `let mut x = 5;` | 可重新赋值 |
| 常量 | `const X: i32 = 5;` | 必须标注类型 |
| 遮蔽 | `let x = 3;` | 用 let 重新声明 |
| 函数 | `fn f(x: i32) -> i32 { x }` | 无 ; 即返回 |
| if 表达式 | `let x = if ... { 1 } else { 0 };` | 分支同类型 |
| 数组 | `[i32; 5]` / `[1; 5]` | 栈上固定大小 |
| 切片 | `&a[1..4]` | 左闭右开 |
| 元组 | `(1, "hi")` | 解构/索引 `.0` |
| Vec | `vec![1,2,3]` | 堆上动态数组 |
| 移动 | `let y = x;` | 所有权转移 |
| 克隆 | `x.clone()` | 深拷贝 |
| 不可变引用 | `&x` | 只读 |
| 可变引用 | `&mut x` | 读写，同一时刻仅一个 |
| 经典结构体 | `struct S { x: u8 }` | 字段名访问 |
| 元组结构体 | `struct T(u8, u8)` | 下标访问 `.0` |
| 单元结构体 | `struct U;` | 无字段 |
| 结构体更新 | `S { x: 1, ..base }` | 继承其余字段 |
| 方法 | `fn m(&self) -> T { }` | 优先 &self |
| 关联函数 | `fn new() -> Self { }` | 类型名调用 |
| 枚举 | `enum E { A, B(u8) }` | 变体可带数据 |
| 模式匹配 | `match v { E::A => ..., E::B(x) => ... }` | 必须穷尽 |

---

# 中篇：核心机制 (9-16)

## 9. Strings（字符串）

### String vs &str

| | String | &str |
|---|---|---|
| 数据在哪 | 堆，自己管理 | 不拥有，指别人的数据或静态区 |
| 可变 | 是 | 否 |
| 栈上大小 | 24 字节 (ptr+len+cap) | 16 字节 (ptr+len) |
| 传参 | 移动所有权 | 不移动，轻量借用 |
| 创建 | `String::from()` / `to_string()` | `"字面量"` / `&s[..]` |

```rust
let mut s1 = String::from("hello");
s1.push_str(" world");         // 能改
s1.push('!');                  // 能追加

let s2: &str = &s1;            // 从 String 借
let s3: &str = "字面量";        // 编译期写死在二进制只读数据区
```

- `String` 是**所有者**，能改
- `&str` 是借来的**视图**，不能改
- 函数参数**优先用 `&str`**，除非需要拿走所有权或修改内容

### 动态大小类型（DST）

`str` 是 DST，大小编译期不知道，**必须藏在指针后面**：

```rust
let s: &str = "hello";              // str 藏在 & 后面
let slice: &[i32] = &arr[0..3];     // [T] 藏在 & 后面
let trait_obj: &dyn ToString = &"hi"; // dyn Trait 藏在 & 后面
```

### 拥有 vs 借用对照表

```
String   : &str      // 拥有字符串  : 字符串切片
Vec<i32> : &[i32]    // 拥有动态数组 : 切片
PathBuf  : &Path     // 拥有路径    : 路径切片
```

核心规律：很多类型成对出现——"拥有版"（堆上可变）和"借用版"（轻量只读视图）。

### &str → String 四种写法

```rust
let s1 = "hello".to_owned();        // ToOwned trait — 意图最明确
let s2 = "hello".to_string();       // Display trait，内部调 to_owned()
let s3 = String::from("hello");     // From trait
let s4: String = "hello".into();    // Into trait，依赖类型推断
```

`to_owned()` 并非字符串专属：`&[i32]` → `Vec<i32>`，`&Path` → `PathBuf`。

### String 的 + 运算符

```rust
let s3 = s1 + &s2;          // s1 被移动走了，&s2 是借用
// + 的签名：fn add(self, rhs: &str) -> String
```

右值能传 `&String` 是因为**解引用强制转换（Deref Coercion）**：`&String` → `&str`。

### 拼接方式对比

| 方式 | 消费左值 | 底层机制 |
|------|---------|---------|
| `s1 + &s2` | 是，s1 被移动 | 复用 s1 的堆缓冲区，少一次分配 |
| `s.push_str(&s2)` | 否（s 需 mut） | 在已有缓冲区末尾写入 |
| `format!("{s1}{s2}")` | 否 | 每次新建缓冲区 |

- 多段拼接**优先用 `format!`**
- 只想追加用 `push_str` 或 `push`

---

## 10. Modules（模块）

### 通俗理解

模块就是代码的"文件夹"——组织代码，形成命名空间。

### 三个核心关键字

| 关键字 | 作用 |
|--------|------|
| `mod` | 定义/声明一个模块 |
| `use` | 把模块路径引入当前作用域 |
| `pub` | 让模块内的东西对外可见（默认私有） |

### 基础用法

```rust
mod animal {
    fn secret() {}        // 默认私有
    pub fn eat() {        // pub 才能被外部调用
        secret();
    }
}
animal::eat();            // :: 路径分隔符
```

### 模块的文件组织

| 方式 | 写法 | 说明 |
|------|------|------|
| 内联 | `mod foo { ... }` | 同一文件 |
| 同目录文件 | `mod foo;` → `foo.rs` | 同名 .rs 文件 |
| 同目录文件夹 | `mod foo;` → `foo/mod.rs` | 同名文件夹 |

### use 的常见写法

```rust
use std::collections::HashMap;      // 单个类型
use std::io::{self, Read, Write};   // 多个，self 指代 io 本身
use std::fs::*;                     // 通配导入（谨慎）
```

### 可见性规则

- 不加 `pub` → 仅在当前模块及子模块可见
- `pub` → 外部可访问
- `pub(crate)` → 当前 crate 内可见
- `pub(super)` → 父模块可见
- `pub(in path)` → 指定路径内可见

### use 的可见性：默认私有

```rust
use b::hello;      // 私有：a 内部能用，外部不能
pub use b::hello;  // 重新导出（re-export）：外部可通过 a::hello 调用
```

### Crate 概念

crate 是 Rust 的**编译单元**：

| 类型 | 入口文件 | 产物 |
|------|----------|------|
| binary crate | `src/main.rs` | 可执行文件 |
| library crate | `src/lib.rs` | `.rlib` 库文件 |

三层体系：

| 层级 | 概念 | 说明 |
|------|------|------|
| package | 包 | 一个 `Cargo.toml` |
| crate | 包箱 | 编译单元 |
| module | 模块 | crate 内部代码组织 |

- `crate::` 从 crate 根出发的绝对路径
- `super::` 往上一层父模块

### 与 Java 的类比

| Java | Rust | 作用 |
|------|------|------|
| `package` | `mod` | 命名空间/组织代码 |
| `class` | `struct` + `impl` | 数据 + 行为 |
| `import` | `use` | 引入路径 |

---

## 11. Hashmaps（哈希映射）

### 创建与插入

```rust
use std::collections::HashMap;

let mut basket = HashMap::new();
basket.insert(String::from("banana"), 2);
```

### HashMap key 必须实现的 trait

`#[derive(Hash, PartialEq, Eq, Debug)]`

| trait | 作用 |
|-------|------|
| `Hash` | 计算哈希值 |
| `PartialEq` | `==` 比较（哈希冲突时判断相等） |
| `Eq` | 标记 `==` 满足自反性 |
| `Debug` | 方便调试（非必须） |

### entry API（最惯用写法）

`entry` 只做**一次**哈希查找，返回 `Entry` 枚举（`Vacant` / `Occupied`）：

```rust
// 只有 key 不存在时才触发 or_insert
map.entry("a").or_insert(99);  // "a" 已存在 → 不动
map.entry("b").or_insert(99);  // "b" 不存在 → 插入

// and_modify：只在已存在时触发
map.entry(key)
   .and_modify(|v| *v += 1)   // 已存在 → +1
   .or_insert(1);              // 不存在 → 插入 1
```

执行逻辑：`Occupied` 走 `and_modify` 跳过 `or_insert`，`Vacant` 跳过 `and_modify` 走 `or_insert`。

### 三种插入方式对比

| 方式 | 推荐度 | 说明 |
|------|-------|------|
| `entry(key).or_insert(val)` | 推荐 | 一次哈希查找 |
| `try_insert(key, val)` | 一般 | 1.70+ 才稳定 |
| `if !contains_key { insert }` | 不推荐 | 两次查找 |

---

## 12. Options（可选值）

### 基本概念

`Option<T>` 只有两个变体：`Some(T)` 和 `None`。Rust 没有 `null`，用 `Option` 替代。

### `if let` 与 `unwrap` 的区别

| | `if let` | `unwrap` |
|---|---|---|
| None 时 | 跳过不执行 | 程序崩溃（panic） |
| 写法 | `if let Some(x) = opt` | `opt.unwrap()` |
| 何时用 | 不确定是否有值 | 确定一定有值 |

```rust
// ✅ if let 自动取出值
if let Some(x) = opt {
    println!("{}", x);
}

// ❌ if let 和 unwrap 不能混用
// if let x = opt.unwrap() { ... }
```

### `ref` 与 `&` 的区别

| | `&` | `ref` |
|---|---|---|
| 用在 | 表达式（值一侧） | 模式匹配（变量一侧） |
| 含义 | "创建这个值的引用" | "把匹配到的值绑定为引用" |

```rust
// 等价
let r = &x;        // & 在表达式侧
let ref r = x;     // ref 在模式侧

// match 中不移动所有权 —— 现代推荐写法（match ergonomics）
match &opt {
    Some(p) => println!("{},{}", p.x, p.y),  // p: &Point 自动推断
    _ => (),
}
```

---

## 13. Error Handling（错误处理）

### 两类错误

- **不可恢复**：`panic!`，直接崩溃
- **可恢复**：`Result<T, E>`，调用方决定怎么处理

### `Result<T, E>` 基础

```rust
enum Result<T, E> { Ok(T), Err(E) }

fn generate_nametag_text(name: String) -> Result<String, String> {
    if name.is_empty() {
        Err(String::from("Empty names aren't allowed"))
    } else {
        Ok(format!("Hi! My name is {name}"))
    }
}
```

### `?` 运算符

`?` 本质是 `match` + `return Err` 的语法糖：

```rust
let qty = item_quantity.parse::<i32>()?;

// 等价于：
let qty = match item_quantity.parse::<i32>() {
    Ok(n) => n,
    Err(e) => return Err(e.into()),
};
```

**使用条件**：`?` 只能在返回 `Result` 或 `Option` 的函数中使用。

### `main` 中使用 `?`

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cost = total_cost("8")?;     // ? 在 main 中直接用
    println!("Tokens: {}", 100 - cost);
    Ok(())
}
```

### `Box<dyn Error>` — 通用错误容器

不同类型错误需要混用时，用 `Box<dyn Error>` 兜底。

### `map_err` — 错误类型转换

只转换 `Err` 分支，`Ok` 原封不动：

```rust
// 伪代码
fn map_err<T, E, F>(self, op: F) -> Result<T, F> {
    match self { Ok(v) => Ok(v), Err(e) => Err(op(e)) }
}

// 枚举变体直接当函数用
s.parse::<i64>().map_err(ParsePosNonzeroError::ParseInt)?;

// 等价于：
s.parse::<i64>().map_err(|e| ParsePosNonzeroError::ParseInt(e))?;
```

### 自定义错误需实现 `Display + Error`

```rust
impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self { ... })
    }
}
impl std::error::Error for CreationError {}
```

### ⚠️ `JoinHandle::join()` 也返回 `Result`

线程可能 panic，`join()` 返回 `Result<T, Box<dyn Any + Send>>`：
- `Ok(T)` — 线程正常结束
- `Err(Box<dyn Any>)` — 线程 panic 了

---

## 14. Generics（泛型）

### 基本概念

泛型是**类型的占位符**，让同一份代码处理不同类型。

### 泛型语法三要素

| 位置 | 写法 | 说明 |
|------|------|------|
| 结构体定义 | `struct Wrapper<T>` | `<T>` 在类型名后 |
| 实现块 | `impl<T> Wrapper<T>` | `impl<T>` 声明参数，`Wrapper<T>` 绑定到类型 |
| 函数 | `fn foo<T>(x: T)` | 同理 |

```rust
struct Wrapper<T> { value: T }

impl<T> Wrapper<T> {
    fn new(value: T) -> Self { Wrapper { value } }
}

let w1 = Wrapper::new(42);    // T = i32（编译器推断）
let w2 = Wrapper::new("Foo"); // T = &str
```

### `Vec<T>` 需要显式标注

```rust
let mut numbers: Vec<i16> = Vec::new();  // 空 Vec 需要标注 T
```

---

## 15. Traits（特征）

### 本质

Trait 定义一组方法签名，描述类型的**共享行为**。类似 Java interface / C++ 抽象类，但可以有**默认实现**。

### 基本语法

```rust
trait Licensed {
    fn licensing_info(&self) -> String;  // 必须实现

    fn default_info(&self) -> String {   // 有默认实现（可选覆盖）
        "Default license".to_string()
    }
}

struct SomeSoftware;
impl Licensed for SomeSoftware {}  // 空实现，使用默认方法
```

### `impl Type` vs `impl Trait for Type`

| 写法 | 含义 | 例子 |
|------|------|------|
| `impl Type { }` | 自身方法（inherent） | `impl String { fn new() ... }` |
| `impl Trait for Type { }` | 实现 trait | `impl Display for Point { ... }` |

### `impl Trait` 作为参数 — 语法糖

```rust
fn compare(software1: impl Licensed, software2: impl Licensed) -> bool {
    software1.licensing_info() == software2.licensing_info()
}
// 等价泛型写法：
fn compare<T: Licensed, U: Licensed>(software1: T, software2: U) -> bool { ... }
```

### 多重 trait 约束

```rust
fn some_func(item: impl SomeTrait + OtherTrait) -> bool { ... }
// 等价：
fn some_func<T: SomeTrait + OtherTrait>(item: T) -> bool { ... }
```

### 孤儿规则 (Orphan Rule)

**只能为"你的"trait 或"你的"类型写实现**：

| impl 组合 | 允许？ |
|-----------|:---:|
| `impl MyTrait for MyType` | ✅ |
| `impl MyTrait for String` | ✅ trait 是你的 |
| `impl Display for MyType` | ✅ type 是你的 |
| `impl Display for String` | ❌ 两边都外来 |

**破局 — Newtype 模式**：用元组结构体包装外来类型。

### 关联类型（Associated Type）

```rust
trait Iterator {
    type Item;                           // 关联类型：实现者决定
    fn next(&mut self) -> Option<Self::Item>;
}

impl Iterator for Counter {
    type Item = u32;                     // 具体化
    fn next(&mut self) -> Option<u32> { ... }
}
```

| 场景 | 用泛型参数 | 用关联类型 |
|------|-----------|-----------|
| 一个类型可以有多种实现 | ✅ | ❌ 只能定死一个 |
| 实现者决定输出类型 | 也行（需标注） | ✅ 简洁 |

> 完整 Trait 速查见 **[专题五：常用 Trait 速查表](#专题五常用-trait-速查表)**

---

## 16. Lifetimes（生命周期）

### 核心概念

生命周期是编译器验证**引用是否有效**的机制。它不改变代码逻辑，不延长变量存活时间——只给借用检查器提供信息。

### 语法：`'a`

```rust
&'a i32        // 带生命周期 'a 的引用
&'a mut i32    // 可变引用同理
```

### 函数中的生命周期

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

含义：x 和 y 至少活得跟 `'a` 一样久，返回值也活得跟 `'a` 一样久。

### 结构体中的生命周期

```rust
struct Book<'a> {
    author: &'a str,
    title: &'a str,
}
```

### 生命周期省略规则（Elision）

| 规则 | 说明 |
|------|------|
| 每个引用参数各自获得一个生命周期 | `fn foo(x: &str)` → `fn foo<'a>(x: &'a str)` |
| 只有一个输入生命周期时，返回值也用这个 | `fn foo(x: &str) -> &str` → 同上 |
| `&self` / `&mut self` 存在时，返回值用 self 的生命周期 | 方法常不用手写 |

### `'static` 生命周期

程序整个运行期间都有效：字符串字面量天生是 `'static`。

### 关键理解：生命周期取交集

多个 `'a` 约束时取**最短的那个**。编译器按最坏情况检查：

```rust
fn main() {
    let string1 = String::from("long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(&string1, &string2);
    }  // ← string2 销毁，result 的 'a 生效期结束
    // println!("{result}");  // ❌ result 引用了已销毁的 string2！
}
```

### `println!` 不会移动所有权

`println!` 宏内部用不可变借用，传 `String` 不会拿走所有权。

---

# 下篇：进阶主题 (17-23)

## 17. Tests（测试）

### 测试模块结构

```rust
#[cfg(test)]           // 只在测试模式编译
mod tests {
    use super::*;     // 从父模块导入待测函数（super = 上一层）

    #[test]
    fn test_something() { ... }
}
```

### 三种断言宏

| 宏 | 用途 | 示例 |
|----|------|------|
| `assert!` | 断言布尔值为 true | `assert!(is_even(2));` |
| `assert_eq!` | 断言相等 | `assert_eq!(power_of_2(2), 4);` |
| `assert_ne!` | 断言不等 | `assert_ne!(x, y);` |

### `#[should_panic]`

测试期望的 panic，panic 发生才算通过：

```rust
#[test]
#[should_panic]
fn negative_width() {
    Rectangle::new(-10, 10);  // 构造时 panic → 测试通过
}

// 精准匹配消息
#[should_panic(expected = "must be positive")]
```

### `#[...]` — 属性（Attribute）

元数据注解，只影响编译过程，不参与运行时：

| 属性 | 作用 |
|------|------|
| `#[test]` | 标记测试函数 |
| `#[should_panic]` | 测试应该 panic |
| `#[cfg(test)]` | 条件编译：只在测试模式 |
| `#[derive(Debug)]` | 自动生成 Debug trait |
| `#[allow(unused)]` | 抑制编译警告 |

三种形式：
```rust
#[test]                              // 无参数
#[should_panic(expected = "...")]    // 键值参数
#[cfg(any(unix, windows))]           // 布尔表达式
```

---

## 18. Iterators（迭代器）

### 三种迭代方式

| 方法 | 产出类型 | 原集合状态 | 场景 |
|------|----------|------------|------|
| `.iter()` | `Option<&T>` | 仍可用 | 只读 |
| `.iter_mut()` | `Option<&mut T>` | 仍可用 | 修改元素 |
| `.into_iter()` | `Option<T>` | 被消耗 | 拿走所有权 |

```rust
let arr = ["a", "b", "c"];
// .iter() → 每个元素是 &T，返回 &&str
let mut it = arr.iter();
assert_eq!(it.next(), Some(&"a"));
// arr 还能继续用 ✅

// .into_iter() → 拿走所有权，返回 T 本身
let mut it2 = arr.into_iter();
assert_eq!(it2.next(), Some("a"));
// println!("{:?}", arr);  // ❌ arr 已被消耗
```

### `.collect()` — 万能收集器

根据返回类型自动收集。同一个 `.collect()` 只因目标类型不同就不同行为：

```rust
// → Result<Vec<_>, _>：全 Ok → Ok(vec)，遇 Err → 短路返回
numbers.into_iter().map(|n| divide(n, 27)).collect()

// → Vec<Result<_, _>>：每个 Result 独立保留
numbers.into_iter().map(|n| divide(n, 27)).collect()
```

### 常用消费方法

| 方法 | 作用 | 空迭代器返回值 |
|------|------|----------------|
| `.sum()` | 求和 | `0` |
| `.product()` | 求积 | `1` |
| `.count()` | 计数 | `0` |
| `.collect()` | 收集到集合 | 空集合 |
| `.fold(init, f)` | 自定义折叠 | 返回 `init` |

### `filter` vs `filter_map`

| 方法 | 闭包返回 | 行为 | 场景 |
|------|----------|------|------|
| `filter(f)` | `bool` | `true` 保留，`false` 丢弃 | 纯过滤 |
| `filter_map(f)` | `Option<T>` | `Some(x)` 保留 x，`None` 丢弃 | 过滤 + 转换 |

### `flat_map` — 摊平嵌套迭代器

```rust
// collection: &[HashMap<_, _>]
collection.iter().flat_map(|m| m.values()).filter(|v| *v == value).count()
```

### `values()` vs `iter()` on HashMap

```
&HashMap<K, V>  →  .values()  →  Iterator<Item = &V>        // 直接拿值
&HashMap<K, V>  →  .iter()    →  Iterator<Item = (&K, &V)>   // 拿键值对
```

### capitalize_first 经典实现

```rust
fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().chain(chars).collect(),
    }
}
// to_uppercase() 返回的是迭代器（一个字符可能变大写后变多个），用 chain 拼接
```

---

## 19. Smart Pointers（智能指针）

### 四种智能指针对比

| 智能指针 | 一句话 | 关键区别 |
|---------|--------|---------|
| `Box<T>` | 数据放堆上 | 唯一所有者，最简单 |
| `Rc<T>` | 多人共享 | 引用计数，单线程 |
| `Arc<T>` | 跨线程共享 | Rc + 原子操作 |
| `Cow<T>` | 能省就省 | 惰性克隆 |

### Box`<T>` — 递归类型救星

```rust
// ❌ 编译错误：recursive type has infinite size
enum List { Cons(i32, List), Nil }

// ✅ Box 打断递归
enum List { Cons(i32, Box<List>), Nil }
```

### Rc`<T>` — 多个所有者

```rust
use std::rc::Rc;

let sun = Rc::new(Sun);              // 计数 = 1
let m = Planet::Mercury(Rc::clone(&sun));  // 计数 = 2，不拷贝数据
let v = Planet::Venus(Rc::clone(&sun));    // 计数 = 3
```

| | `&T`（引用） | `Rc<T>` |
|---|---|---|
| 所有权 | 借用，不拥有 | 共享所有权 |
| 生存期 | 受生命周期约束 | 运行时动态管理 |
| 线程 | ✅ | ❌ 单线程 |

- **`Rc::clone(&rc)`** — 只增加计数，不拷贝数据（极轻量）
- **`Rc::strong_count(&rc)`** — 查看当前计数
- **`drop(x)`** — 手动提前释放，减少计数
- 还有弱引用 `Rc::weak_count`，通过 `Rc::downgrade` 创建
- `Rc::new(value)` 参数走 move 语义

### Arc`<T>` — 线程安全的 Rc

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

### Cow`<T>` — 写时克隆

先用别人的数据，真要改时才复制：

```rust
use std::borrow::Cow;

fn abs_all(input: &mut Cow<[i32]>) {
    for i in 0..input.len() {
        if input[i] < 0 {
            input.to_mut()[i] = -input[i];  // 需要修改时才克隆！
        }
    }
}

// 全正数 → 不克隆，Cow::Borrowed
let mut cow = Cow::from(&vec![1, 2, 3]);
abs_all(&mut cow);

// 有负数 → 自动克隆，Cow::Owned
let mut cow = Cow::from(&vec![-1, 2, 3]);
abs_all(&mut cow);
```

- `to_mut()` 返回 `&mut T`。Borrowed 变 Owned（克隆），Owned 直接改
- Deref 让 Cow 读时零开销，但写必须通过 `to_mut()`——不能直接通过 `&mut Cow` 改内部

### dyn Trait 与 Box`<dyn Trait>`

`dyn` = 动态分发，表示"实现了某 trait 的任意类型"：

| | 静态分发 `<T: Trait>` | 动态分发 `dyn Trait` |
|---|---|---|
| 调用方式 | 编译时跳转（单态化） | 运行时虚表查找 |
| 性能 | 更快 | 轻微虚表开销 |
| 二进制大小 | 每种类型一份（更大） | 只一份 |
| 集合异构 | 不能 | 能 |

```rust
// 异构集合
let animals: Vec<Box<dyn Animal>> = vec![
    Box::new(Dog), Box::new(Cat), Box::new(Dog),
];
```

### 缩写含义

| 缩写 | 全称 | 含义 |
|------|------|------|
| `Box` | 不是缩写 | 数据装进堆上"盒子" |
| `Rc` | Reference Counted | 引用计数 |
| `Arc` | Atomic Reference Counted | 原子引用计数 |
| `Cow` | Clone on Write | 写时克隆 |

---

## 20. Threads（线程）

### 线程创建：`thread::spawn`

```rust
let handle = thread::spawn(move || {
    // 新线程代码
    42  // 返回值
});
let result = handle.join().unwrap();  // 阻塞等待，取回结果
```

| 概念 | 说明 |
|---|---|
| `thread::spawn` | 创建新线程 |
| `JoinHandle` | 线程"遥控器" |
| `join()` | 阻塞等待，返回 `Result<T>`（线程 panic 则 Err） |
| `move` 闭包 | 强制获取外部变量所有权 |

### `move` 闭包详解

新线程可能比当前作用域活得更久，闭包必须**持有**而非借用外部变量：

```rust
// ❌ 不带 move：闭包尝试借用，生命周期不够
// thread::spawn(|| { println!("{s}"); });

// ✅ 带 move：s 的所有权移入闭包
thread::spawn(move || { println!("{s}"); });
// println!("{s}");  // ❌ s 已被 move 走
```

`Copy` 类型下 `move` 相当于复制一份，每个线程有自己的副本。

### 闭包三种捕获方式

| 捕获方式 | Trait | 说明 |
|---|---|---|
| 不可变借用 | `Fn` | 只读，可多次调用 |
| 可变借用 | `FnMut` | 修改，可多次调用 |
| 拿走所有权 | `FnOnce` | `move` 进来后只能调一次 |

### 共享可变状态：`Arc<Mutex<T>>`

| 组件 | 解决问题 |
|---|---|
| `Arc<T>` | 多个线程共享同一份数据 |
| `Mutex<T>` | 互斥访问，同一时刻只有一个线程能修改 |

```rust
use std::sync::{Arc, Mutex};

let status = Arc::new(Mutex::new(JobStatus { jobs_done: 0 }));

// 子线程传递
let shared = Arc::clone(&status);
thread::spawn(move || {
    shared.lock().unwrap().jobs_done += 1;  // lock → 改 → 离开作用域自动解锁
});

// 主线程读取
println!("{}", status.lock().unwrap().jobs_done);
```

`lock()` → `Result<MutexGuard<T>>` → `unwrap()` → 当 `&mut T` 用 → 离开作用域 `Drop` 释放锁。

### 消息传递：`mpsc::channel`

mpsc = **m**ulti-**p**roducer, **s**ingle-**c**onsumer：

```rust
use std::sync::mpsc;

let (tx, rx) = mpsc::channel();

let tx1 = tx.clone();                 // Sender 可 clone
thread::spawn(move || { tx1.send(42).unwrap(); });
thread::spawn(move || { tx.send(100).unwrap(); });

for value in rx {                     // 阻塞等待，所有 Sender drop 后结束
    println!("{value}");              // 42 和 100（顺序不定）
}
```

### 共享内存 vs 消息传递

| | `mpsc::channel` | `Arc<Mutex<T>>` |
|---|---|---|
| 理念 | 传递消息，不共享内存 | 共享内存，靠锁保护 |
| 数据流向 | 单向：多S → 1R | 任意线程读写 |
| 同步开销 | 无锁 | 有锁竞争 |
| 适合场景 | 生产者-消费者、流水线 | 多线程读写同一状态 |

---

## 21. Macros（宏）

### 宏是什么

宏是**写代码的代码**——编译时把简写展开成完整 Rust 代码。宏调用必须带 `!`。

### 基本语法

```rust
macro_rules! 宏名 {
    (模式) => { 展开体 };
    (另一个模式) => { 另一个展开体 };
}
```

### 带参数的宏

```rust
macro_rules! my_macro {
    ($val:expr) => {
        println!("Look at this: {}", $val);
    };
}

my_macro!(7777);  // 展开为 println!(..., 7777);
```

- `$val` — 元变量（`$` 开头）
- `:expr` — 片段分类符

### 重复匹配

```rust
macro_rules! sum {
    ($($x:expr),+) => {{
        let mut total = 0;
        $(total += $x;)+  // 对每个匹配到的 $x 重复这段代码
        total
    }};
}
```

| 重复运算符 | 含义 |
|-----------|------|
| `*` | 零次或多次 |
| `+` | 一次或多次 |
| `?` | 零次或一次 |

### 常用片段分类符

| 分类符 | 匹配内容 |
|--------|----------|
| `expr` | 表达式 |
| `ident` | 标识符 |
| `ty` | 类型 |
| `tt` | 单个 token 树（最灵活） |
| `literal` | 字面量 |
| `stmt` | 语句 |
| `pat` | 模式 |

### 宏的作用域

宏定义默认**私有**，且**必须先定义后使用**：

```rust
mod macros {
    macro_rules! my_macro { ... };  // 默认只在模块内可见
}

// 三种导出方式：
#[macro_use]     mod macros { ... }  // 一口气导出所有
use macros::my_macro;                // 精确导入（推荐）
#[macro_export]  macro_rules! ...    // 跨 crate 可见
```

### 宏 vs 函数

| | 函数 | 宏 |
|---|---|---|
| 参数数量 | 固定 | 可变（`+`/`*`） |
| 参数类型 | 固定 | 任意匹配 |
| 可变参数 | 不支持 | 天然支持 |
| 展开时机 | 运行时 | 编译时 |

---

## 22. Clippy（代码检查工具）

### 什么是 Clippy

Rust 官方 lint 工具，分析代码并给出改进建议。安装：`rustup component add clippy`

### 常见 Clippy 建议

**1. 直接用常量，不要重复绑定**

```rust
// ❌ let pi = std::f32::consts::PI;
// ✅ let area = std::f32::consts::PI * radius.powi(2);
```

**2. 单次匹配用 `if let`，不要用 `while let`**

```rust
// ❌ while let Some(x) = option { res += x; }
// ✅ if let Some(x) = option { res += x; }
```

**3. 用 `std::mem::swap` 代替手动交换**

```rust
// ❌ let temp = a; a = b; b = temp;
// ✅ use std::mem::swap; swap(&mut a, &mut b);
```

**4. 变量类型与命名一致，不要随意 `unwrap`**

---

## 23. Conversions（类型转换）

### 五种转换方式总览

| 方式 | trait | 可能失败 | 典型场景 |
|------|-------|:---:|----------|
| `as` | 语言内置 | ❌ | 基本数值转换 |
| `From` / `Into` | `From`, `Into` | ❌ | 可靠的值转值 |
| `FromStr` / `.parse()` | `FromStr` | ✅ | 字符串解析 |
| `TryFrom` / `TryInto` | `TryFrom`, `TryInto` | ✅ | 可失败的转换 |
| `AsRef` / `AsMut` | `AsRef`, `AsMut` | ❌ | 零开销引用借用 |

### Turbofish 语法 `::<>`

泛型方法调用时显式指定类型参数：

```rust
"42".parse::<u8>().unwrap();   // ✅ turbofish
//  "42".parse<u8>().unwrap(); // ❌ 编译器误解为 parse < u8
let n: u8 = "42".parse().unwrap();  // ✅ 从变量推断
```

| 形式 | 含义 | 示例 |
|------|------|------|
| `Type<u8>` | 类型声明 | `Vec<u8>` |
| `func::<u8>()` | 泛型方法调用 | `parse::<u8>()` |

### `From` & `Into` — 可靠转换

**实现 `From` 就够了，`Into` 自动获得**：

```rust
impl From<&str> for Person {
    fn from(s: &str) -> Self { ... }
}

let p1 = Person::from("Mark,20");     // From
let p2: Person = "Gerald,70".into();  // Into（自动获得）
```

### `FromStr` & `.parse()` — 字符串解析

```rust
impl FromStr for Person {
    type Err = ParsePersonError;
    fn from_str(s: &str) -> Result<Self, Self::Err> { ... }
}

let p = "Mark,20".parse::<Person>().unwrap();
```

### `TryFrom` & `TryInto` — 可失败转换

```rust
impl TryFrom<(i16, i16, i16)> for Color {
    type Error = IntoColorError;
    fn try_from(tuple: (i16, i16, i16)) -> Result<Self, Self::Error> {
        Ok(Color {
            red: u8::try_from(tuple.0).map_err(|_| IntConversion)?,
            // u8::try_from 要求值在 0..=255
        })
    }
}
let c: Result<Color, _> = (183, 65, 14).try_into();
```

### `AsRef` & `AsMut` — 零开销引用转换

**不获取所有权、不拷贝数据**，只借出一个引用：

| 维度 | `From` / `Into` | `AsRef` / `AsMut` |
|------|----------------|-------------------|
| 所有权 | 消耗或新建 | 不消耗，只借用 |
| 开销 | 可能有 | **零开销** |
| 返回 | `Self` | `&T` 或 `&mut T` |

```rust
// AsRef：泛型函数通吃 &str 和 String
fn byte_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().len()
}
byte_counter("hello");              // &str → 直接借用
byte_counter(String::from("hi"));   // String → 零开销借出 &str

// AsMut：可变借用
fn num_sq<T: AsMut<u32>>(arg: &mut T) {
    let n = arg.as_mut();
    *n = *n * *n;
}
```

### `&` vs `as_ref()`

| | `&` | `as_ref()` |
|---|---|---|
| 本质 | 语言运算符 | trait 方法 |
| 类型 | `T → &T`，不变 | 可以变 `String → &str` |
| 泛型 | 不灵活 | 多态，同一调用适配多种类型 |

### `AsMut` 为什么参数必须是 `&mut self`

要返回 `&mut T`，前提是独占访问内部数据。`&self` 只给只读权，无法保证没有别人在看同一块内存：

```
as_ref(): &self     → &T       // 只读借只读 ✅
as_mut(): &mut self → &mut T   // 可变借可变，必须独占
```

### 一揽子实现（Blanket Impl）

标准库为 `&mut T` 做了转发：只要 `T` 实现了 `AsMut<U>`，`&mut T` 自动也获得 `AsMut<U>`。这就是为什么 `&mut Box<u32>` 可以直接调 `as_mut()`。

---

# 专题合集

## 专题一：Rust 控制流全解

### 1. if / else if / else — 条件分支

`if` 是**表达式**，可直接赋值：

```rust
let x = 5;
let y = if x > 0 { 1 } else { -1 };   // 所有分支必须返回同类型
```

**if 与 else 类型必须一致**：

```rust
let n = if true { 1 } else { 0 };    // ✅ 两边都是 i32
// let n = if true { 1 } else { "零" }; // ❌ 类型不匹配
```

**省略 else 时隐含返回 `()`**：

```rust
let result = if x > 10 { "大" };     // ⚠️ else 隐含返回 ()
```

**多条件链**：

```rust
let grade = if score >= 90 { 'A' }
           else if score >= 80 { 'B' }
           else if score >= 70 { 'C' }
           else { 'F' };
```

### 2. loop — 无限循环

**`break` 可以带返回值**：

```rust
let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2;  // loop 的返回值
    }
};
println!("{}", result);  // 20
```

**循环标签（Label）** — 用于嵌套循环中指定目标：

```rust
'outer: loop {
    println!("外层");
    loop {
        println!("  内层");
        break 'outer;    // 跳出外层，而非内层
    }
}

// 配合 continue：
'outer: for x in 0..5 {
    for y in 0..5 {
        if y == 2 { continue 'outer; }  // 跳过外层本次迭代
        if x == 3 && y == 3 { break 'outer; }
    }
}
```

### 3. while — 条件循环

```rust
let mut number = 3;
while number != 0 {
    println!("{}!", number);
    number -= 1;
}
```

**while let** — 模式匹配成功时持续循环：

```rust
let mut stack = vec![1, 2, 3];
while let Some(top) = stack.pop() {
    println!("{}", top);  // 3, 2, 1
}
```

### 4. for — 迭代器循环（最惯用）

```rust
// 遍历元素（不可变引用）
for element in a.iter() { ... }

// 遍历元素（可变引用）
for element in v.iter_mut() { *element += 1; }

// 范围遍历
for number in 1..4 { }      // [1, 2, 3] 左闭右开
for number in 1..=4 { }     // [1, 2, 3, 4] 闭区间
for number in (1..4).rev()  // 3, 2, 1 反向
```

`for x in arr` 等价于 `for x in arr.into_iter()`。`for x in &arr` 等价于 `for x in arr.iter()`。

### 5. match — 模式匹配

```rust
let x = 2;
match x {
    1 => println!("一"),
    2 => println!("二"),
    3 => println!("三"),
    _ => println!("其他"),  // 通配符
}
```

**解构**：

```rust
enum Coin { Penny, Nickel, Dime, Quarter(u8) }

match coin {
    Coin::Quarter(year) => {  // 解构出 year
        println!("{}年铸造", year);
        25
    }
    _ => 0,
}
```

**`_` 通配符 vs 变量绑定**：

```rust
match value {
    1 => do_something(),
    _ => do_default(),    // _ 匹配一切但不绑定
}

match value {
    1 => do_something(),
    other => {            // other 绑定匹配到的值
        println!("{}", other);
    }
}
```

**`|` 匹配多个值**：

```rust
match x {
    1 | 2 | 3 => println!("一到三"),
    _ => println!("其他"),
}
```

**匹配守卫（Match Guard）**：

```rust
let pair = (2, -2);
match pair {
    (x, y) if x == y => println!("相等"),
    (x, y) if x + y == 0 => println!("和为0"),
    _ => println!("其他"),
}
```

### 6. if let — 简洁模式匹配

```rust
// match 写法
match some_value {
    Some(3) => println!("是三"),
    _ => (),
}

// if let 写法（等价，更简洁）
if let Some(3) = some_value {
    println!("是三");
}

// if let 带 else
if let Some(value) = optional {
    println!("有值: {}", value);
} else {
    println!("无值");
}
```

### 7. break 和 continue

| 关键字 | 作用 | 适用范围 |
|--------|------|---------|
| `break` | 立即退出循环 | `loop`, `while`, `for` |
| `continue` | 跳过本次剩余代码 | `loop`, `while`, `for` |

`break` 在 `loop` 中可带返回值。

### 8. 控制流对比表

| 场景 | 推荐写法 | 原因 |
|------|---------|------|
| 二选一分支 | `if / else` | 直接 |
| 多分支根据值判断 | `match` | 穷尽检查 |
| 只关心一种匹配 | `if let` | 简洁 |
| 无限循环 + 内部跳出 | `loop` | break 可带返回值 |
| 有明确终止条件 | `while` | 语义清晰 |
| 遍历集合/范围 | `for` | 最安全 |
| 遍历时修改元素 | `for ... in ... iter_mut()` | 标准 |
| 嵌套循环精确跳转 | 带标签的 `break 'label` | 避免歧义 |

### 9. 与 C/Java 的关键差异

| 概念 | C / Java | Rust |
|------|----------|------|
| `if` | 语句，不能赋值 | 表达式，可以赋值 |
| `switch` | 有 fall-through | `match` 无 fall-through，自动穷尽 |
| 三元 `?:` | `a ? b : c` | 直接用 `if a { b } else { c }` |
| 数组越界 | 运行时不检查 | `for .. in` 自动避免 |
| C风格 `for` | `for (i=0; i<n; i++)` | `for i in 0..n` |
| `while let` | 无 | 有 |

---

## 专题二：let 模式解构详解

### `let` 即模式匹配

Rust 中 `let` 语句本身就是一个模式匹配：

```rust
let x = 5;                    // 模式 x — 匹配任何值
let (a, b) = (1, 2);         // 模式 (a, b) — 解构元组
let Point { x, y } = p;      // 模式 Point { x, y } — 解构结构体
```

### irrefutable vs refutable

| 模式类型 | 含义 | 能用在哪 | 示例 |
|----------|------|---------|------|
| **irrefutable**（不可反驳） | 一定能匹配上 | `let`、函数参数、`for` | `let (a, b) = tup;` |
| **refutable**（可反驳） | 可能匹配不上 | `if let`、`while let`、`match` | `if let Some(x) = opt` |

核心规则：`let` 要求模式是不可反驳的——编译器必须证明模式**永远**匹配成功。

```rust
// ✅ 不可反驳
let Point { x, y } = p;    // 结构体解构永远成功
let (a, b, c) = tup;       // 元组解构永远成功

// ❌ 可反驳 — let 不接受
// let Some(x) = opt;       // Option 不一定是 Some

// ✅ 改用 if let
if let Some(x) = opt { println!("{}", x); }
```

### 结构体解构的三种写法

```rust
struct Point { x: i32, y: i32 }
let p = Point { x: 10, y: 20 };

// 1. 简写：变量名与字段名相同
let Point { x, y } = p;

// 2. 重命名：字段名: 变量名
let Point { x: my_x, y: my_y } = p;

// 3. 忽略部分字段：..
let Point { x, .. } = p;
```

### 枚举解构（可反驳，须用 if let 或 match）

```rust
enum Message { Quit, Move { x: i32, y: i32 }, Write(String) }

let msg = Message::Move { x: 10, y: 20 };

// if let 解构
if let Message::Move { x, y } = msg {
    println!("移动到 ({}, {})", x, y);
}

// match 穷尽解构
match msg {
    Message::Quit => println!("退出"),
    Message::Move { x, y } => println!("移动到 ({}, {})", x, y),
    Message::Write(s) => println!("写入: {}", s),
}
```

### 函数参数也是模式

```rust
fn print_point(Point { x, y }: &Point) {  // 直接在参数位置解构
    println!("({}, {})", x, y);
}

fn swap((a, b): (i32, i32)) -> (i32, i32) {  // 解构元组参数
    (b, a)
}
```

### 各场景适用的模式类型

| 写法 | 模式类型要求 | 匹配不上时 |
|------|:----------:|---------|
| `let PAT = expr;` | 不可反驳 | 编译错误 |
| `fn f(PAT: Type)` | 不可反驳 | 编译错误 |
| `if let PAT = expr { }` | 可反驳 | 跳过 if 体 |
| `while let PAT = expr { }` | 可反驳 | 循环终止 |
| `match expr { PAT => ... }` | 可反驳（需穷尽） | 尝试下一分支 |

### 与关系

`let`、`if let`、`while let` 本质是同一套模式匹配机制，区别在于**对匹配失败的容忍度**：

```
let x      =   expr   → 必须匹配，否则编译报错
if let x   =   expr   → 匹配不上就跳过
while let x =   expr  → 匹配不上就退出循环
```

---

## 专题三：范围类型 (Range) 详解

### 两种 Range 类型

| 语法 | 类型名 | 区间 | `contains` 逻辑 |
|------|--------|------|----------------|
| `a..b` | `Range` | `[a, b)` 左闭右开 | `a <= x < b` |
| `a..=b` | `RangeInclusive` | `[a, b]` 双闭 | `a <= x <= b` |

其他：
- `..b` = `RangeTo`，`x < b`
- `a..` = `RangeFrom`，`a <= x`
- `..` = `RangeFull`，表示一切

### `contains()` — 判断值是否在范围内

```rust
if (18..60).contains(&age) { println!("成年人"); }
if (1..=100).contains(&age) { println!("在 1 到 100 之间"); }
if ('a'..='z').contains(&'c') { println!("小写字母"); }

// match 守卫
match age {
    n if (0..=17).contains(&n) => println!("未成年"),
    n if (18..=59).contains(&n) => println!("成年"),
    _ => println!("老年"),
}
```

注意必须传 `&x`，因为 `contains` 签名是 `fn contains(&self, item: &T) -> bool`。

### 适用类型

要求 `PartialOrd` + `PartialEq`：`i32`、`f64`、`char` 等都可用。`String`/`&str` 不适用。

### `for` 循环中

```rust
for i in 0..5 { }    // 0, 1, 2, 3, 4（不含 5）
for i in 0..=5 { }   // 0, 1, 2, 3, 4, 5（含 5）
```

### Range 是迭代器

```rust
let squares: Vec<i32> = (1..=5).map(|x| x * x).collect();  // [1, 4, 9, 16, 25]
let sum: i32 = (1..100).sum();  // 4950
for i in (1..=5).rev() { }     // 5, 4, 3, 2, 1
```

### 对比总结

| 场景 | 推荐语法 |
|------|---------|
| 判断值在范围内 | `(low..=high).contains(&x)` |
| `for` 遍历 n 次 | `0..n` |
| `for` 遍历 1 到 n | `1..=n` |
| 字符范围判断 | `('a'..='z').contains(&c)` |
| 切片索引 | `&arr[0..5]`（切片不支持 `..=`） |

---

## 专题四：Deref 解引用深度剖析

### `*` 解引用运算符

`*` 的作用：**顺着引用找到实际的值**。类比 C 语言的 `*p`。

```rust
let x = 42;
let r = &x;
let y = *r;        // 顺着 r 找到盒子里的 42
```

### `Deref` trait

`*` 运算符解引用需要类型实现 `Deref` trait：

```rust
pub trait Deref {
    type Target;
    fn deref(&self) -> &Self::Target;
}
```

`Box<T>`、`Rc<T>`、`Arc<T>`、`Cow<T>`、`String`、`Vec<T>` 都实现了 `Deref`。

### Box 解引用与所有权

```rust
let b = Box::new(42);
let n: i32 = *b;    // i32 是 Copy，拷贝一份，b 仍可用

let b = Box::new(String::from("hi"));
let s: String = *b; // String 不是 Copy，所有权移出
// println!("{b}"); // ❌ b 已失效
```

### 自动解引用强制转换（Deref Coercion）

编译器在以下场景自动插入 `*` 和 `&`：

| 场景 | 转换 | 示例 |
|------|------|------|
| `&String` → `&str` | `&T` → `&U` if `T: Deref<Target=U>` | `&s` 当 `&str` 用 |
| `&mut T` → `&mut U` | 同上，mut 版本 | 可变引用穿透 |
| `&T` 的方法调用 | 自动解到匹配的方法 | `s.trim()`，自动 `&String` → `&str` |

```rust
let b = Box::new(String::from("hi"));
println!("{}", b);  // 不需要 *b！编译器自动解了 &Box → &String → &str
```

### 各类型的 Deref 目标

| 类型 | Deref::Target | 解释 |
|------|--------------|------|
| `String` | `str` | 所以 `&String` 能转 `&str` |
| `Vec<T>` | `[T]` | 所以 `&Vec<T>` 能转 `&[T]` |
| `Box<T>` | `T` | 所以 `*Box<T>` 拿到 `T` |
| `Cow<[i32]>` | `[i32]` | 读时零开销 |
| `MutexGuard<T>` | `T` | 所以 `lock()` 后可以当 `&mut T` |

### Cow 的 Deref 特殊性

`Cow` 实现了 `Deref<Target = [i32]>`，读时自动解引用：

```rust
fn abs_all(input: &mut Cow<[i32]>) {
    input.len();      // 自动解引用 → (*input).len()
    input[ind];       // 自动解引用 → (*input)[ind]
    input.to_mut()[ind] = -value;  // 写必须显式调用 to_mut()！
}
```

`&mut Cow<[i32]>` 的 `mut` 只能替换整个 Cow 的值，不能直接修改内部 `[i32]`：

```rust
// cow[0] = 5;     // ❌ Deref 只给不可变引用
cow.to_mut()[0] = 5;  // ✅ to_mut() 返回 &mut [i32]
```

---

## 专题五：常用 Trait 速查表

| Trait | 用途 | 关联语法 | 获取方式 |
|-------|------|---------|---------|
| `Clone` | 显式复制 | `.clone()` | `#[derive(Clone)]` 或手动 |
| `Copy` | 隐式按位复制（标记 trait） | 赋值/传参 | `#[derive(Copy, Clone)]` |
| `Debug` | 调试打印 | `{:?}` / `{:#?}` | `#[derive(Debug)]` |
| `Display` | 用户友好打印 | `{}` | 手动实现 |
| `PartialEq` | 比较相等 | `==`, `!=` | `#[derive(PartialEq)]` |
| `Eq` | 等号自反性保证 | — | `#[derive(Eq)]`（需先有 PartialEq） |
| `PartialOrd` | 比较大小 | `<`, `>`, `<=`, `>=` | `#[derive(PartialOrd)]` |
| `Hash` | 计算哈希值 | 配合 HashMap | `#[derive(Hash)]` |
| `Drop` | 析构时自动调用 | 离开作用域 | 手动实现 `fn drop(&mut self)` |
| `Default` | 提供默认值 | `T::default()` | `#[derive(Default)]` |
| `From` / `Into` | 类型转换 | `T::from(x)` / `x.into()` | 实现 From，Into 自动获得 |
| `TryFrom` / `TryInto` | 可失败转换 | `T::try_from(x)?` | 实现 TryFrom，TryInto 自动获得 |
| `AsRef` | 零开销不可变借用 | `.as_ref()` | 手动实现 |
| `AsMut` | 零开销可变借用 | `.as_mut()` | 手动实现 |
| `Deref` | 解引用 `*` 操作 | `*x` | 手动实现 |
| `DerefMut` | 可变解引用 | `*x = ...` | 手动实现 |
| `FromStr` | 字符串解析 | `.parse::<T>()` | 手动实现 |
| `Iterator` | 迭代器 | `.next()`, `.map()`, ... | 手动实现 |
| `Fn` / `FnMut` / `FnOnce` | 闭包 trait | 闭包自动实现 | 编译器推导 |

### 常见 trait 组合

```rust
// 完整的"可比较可打印可克隆"值类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]

// HashMap key
#[derive(Hash, PartialEq, Eq)]

// 简单的 Debug + Clone
#[derive(Debug, Clone)]
```

### Copy 的实现条件

`Copy` 是标记 trait，不能随便加。类型必须满足：
1. 所有字段都是 `Copy`
2. 没有实现 `Drop`
3. 不含堆上数据（通常只有数值、char、bool、引用等）

```rust
#[derive(Copy, Clone)]  // Copy 要求 Clone
struct Point { x: i32, y: i32 }  // ✅ 全是 i32

// #[derive(Copy, Clone)]
// struct MyString(String);       // ❌ String 不是 Copy
```

### From/Into 的设计原则

**单向实现**：实现 `From<A> for B`，编译器自动为 A 生成 `Into<B>`。永远只需要实现 `From`。

---

> **至此，Rustlings 全部 24 个练习（0-23 + quizzes）的笔记均已完成。**
