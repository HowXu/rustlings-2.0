# Rustlings 练习笔记 (Exercise 0-10)

> 提取自 `exercises/00_intro` ~ `exercises/08_enums` 的注释与文档。

---

## 0. Intro（入门）

### 宏（Macros）

Rust 使用 `print!` 和 `println!` 宏向控制台输出文本。

### 练习流程

- 完成后输入 `n` 进入下一练习。
- 修改文件后会自动重新加载。

---

## 1. Variables（变量）

### 基本规则

- **变量默认不可变**。一旦值绑定到名称，就不能更改。
- 加 `mut` 可使变量变为可变的：`let mut x = 3;`

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

> `let` 本质是地址与变量的绑定——可以获取只读或可读可写的访问权限。

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

- `fn f(mut x: T)` — 改的是函数内的局部副本，调用方不需要 `mut`
- `fn f(x: &mut T)` — 改的是调用方原来的值，调用方变量必须是 `let mut`

### Copy 语义

实现了 `Copy` trait 的类型（如 `i32`、`bool`）赋值时自动复制，原变量仍可用。复杂类型（`String`、`Vec` 等堆上数据）默认移动，原变量失效。想用克隆就显式 `.clone()`。

---

## 2. Functions（函数）

### 函数签名与返回

- 参数必须标注类型：`fn call_me(num: i64)`
- 返回值用 `->` 标注：`fn square(num: i32) -> i32`

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

### if 是表达式

`if` 是表达式，可以直接用于赋值：

```rust
let identifier = if animal == "crab" {
    1
} else if animal == "gopher" {
    2
} else {
    0
};  // 注意这里需要 ;
```

- 所有分支必须返回相同类型
- 如果只有 `if` 没有 `else`，分支返回 `()`（单元类型）

### 字符串比较

使用 `==` 比较，如 `if food == "strawberry"`

### 测试框架

`#[cfg(test)]` + `mod tests` 是 Rust 内置测试框架，`assert_eq!` 用于断言。测试代码不会编译到正式构建中。

---

## 4. Primitive Types（原始类型）

### bool（布尔值）

```rust
let is_morning: bool = true;
```

### char（字符）

使用**单引号**（不同于字符串的双引号）：`let c = 'A';`

支持 `is_alphabetic()`、`is_numeric()` 等方法。

### 数组（Array）

栈上固定大小：

```rust
let a: [i32; 100] = [5; 100];  // 100个元素，全部初始化为5
//        ^类型^容量      ^值^数量
```

### 切片（Slice）

```rust
let nice_slice = &a[1..4];  // 左闭右开区间 [1, 4)
```

### 元组（Tuple）

```rust
let cat = ("Furry McFurson", 3.5);

// 解构
let (name, age) = cat;

// 索引访问
let second = numbers.1;       // 从 0 开始

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
let v: Vec<i32> = Vec::new(); // 空向量
```

### 可变性

`Vec` 默认不能 `push` / 删除元素，必须声明为 `mut`：

```rust
let mut output = Vec::new();
output.push(42);
```

### 迭代器

```rust
// map 风格
input.iter().map(|element| element + 1).collect()
//                        ^参数声明  ^映射函数    ^收集为 Vec

// 循环风格
for element in input {
    output.push(element * 2);
}
```

- `iter()` 返回只读迭代器
- `|element|` 是闭包（closure）参数声明
- `collect()` 默认收集为 `Vec`

---

## 6. Move Semantics（移动语义）

### 核心规则

**一个值同时只能有一个所有者。** 赋值、传参、返回值时，所有权会转移，原变量失效。

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

需要保留原变量时，显式使用 `.clone()` 深拷贝：

```rust
let vec1 = fill_vec(vec0.clone());   // vec0 仍可用
```

### let mut vec = vec; 发生了什么？

这是**移动语义**，不是深拷贝也不是浅拷贝：
- 底层仅复制栈上的 3 个字段（指针、长度、容量）
- 堆上的数据没有复制
- 原绑定被编译器标记为失效，防止 double-free

### mut 在参数上的含义

`mut` 是**绑定的属性**，不是类型的一部分：

```rust
fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(88);  // mut 允许我们修改这个局部绑定
    vec
}
```

即使传入的是不可变绑定的值，所有权转移后函数内部可以用 `mut` 声明新的可变绑定来持有同一个值并修改它——本质上是同一个 `Vec`，只是不同作用域有不同的可变性声明。

### 可变借用规则

**同一时刻只能有一个 `&mut` 引用**（防止数据竞争）：

```rust
let mut x = Vec::new();
let z = &mut x;  // 拿走可变借用
z.push(42);      // z 的最后一次使用，借用结束
let y = &mut x;  // 现在可以再拿了
y.push(13);
```

### 所有权 vs 引用

```rust
// 只借不拿走 → 传引用
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// to_uppercase 返回新 String，用引用即可
fn string_uppercase(data: &String) {
    let data_ref = data.to_uppercase();
    println!("{data_ref}");
}
```

### . 操作符的自动借用

```rust
v.push(4)   // → 方法调用自动取引用 → &mut v → 传给 push(&mut self, ...)
```

`.` 操作符会自动取 `&` 或 `&mut`，但不能无中生有——`&mut` 只能从 `mut` 绑定生成。`let v` 不写 `mut`，就是声明"这个绑定不给 `&mut` 权限"，后续所有需要 `&mut` 的操作都走不通。

### 权限系统总结

Rust 所有关于"能不能改"的问题，归根结底就是一句话：**你当前通过哪个名字、以什么权限访问那个地址。`mut` 就是这个权限的通行证。**

---

## 7. Structs（结构体）

### 三种结构体

```rust
// 经典结构体（类似 C/JSON）
struct ColorRegularStruct {
    red: u8,
    green: u8,
    blue: u8,
}

// 元组结构体（用下标访问）
struct ColorTupleStruct(u8, u8, u8);

// 单元结构体（无字段，值就是名字本身）
#[derive(Debug)]   // 需要 Debug 才能用 {:?} 打印
struct UnitStruct;
```

### 实例化

```rust
// 经典结构体 — 类似 JSON 的字段名:值写法
let green = ColorRegularStruct { red: 0, green: 255, blue: 0 };

// 元组结构体 — 按位置传值
let green = ColorTupleStruct(0, 255, 0);

// 单元结构体 — 值就是名字
let unit = UnitStruct;
println!("{:?}", unit);    // UnitStruct
```

访问字段：经典结构体用 `.field_name`，元组结构体用 `.0`、`.1`、`.2`。

### 结构体更新语法

```rust
let your_order = Order {
    name: String::from("Hacker in Rust"),
    count: 1,
    ..order_template   // 其余字段从 order_template 继承
};
```

### 方法（impl）

```rust
impl Package {
    // 关联函数（静态方法 / 工厂）：无 &self，用 Self 返回类型
    fn new(sender_country: String, recipient_country: String, weight_in_grams: u32) -> Self {
        Self { sender_country, recipient_country, weight_in_grams }
    }

    // 实例方法：&self 借用实例（多数情况应该用 &self 而非拿走所有权）
    fn is_international(&self) -> bool {
        self.sender_country != self.recipient_country
    }

    fn get_fees(&self, cents_per_gram: u32) -> u32 {
        cents_per_gram * self.weight_in_grams
    }
}
```

- **无 `self` 参数**（如 `fn new() -> Self`）是关联函数（静态方法），调用时用 `Package::new(...)`
- **`&self` 参数**是实例方法，调用时用 `package.method()`，`.` 操作符自动取引用
- **`Self`** 关键字指代 `impl` 所在的类型名（即 `Package`）
- **优先用 `&self`**：借用不会触发所有权转移，避免值被移走导致的后续不可用

### `#[derive(Debug)]`

自动实现 `Debug` trait，允许用 `{:?}` 调试打印。如需美化输出用 `{:#?}`。

---

## 8. Enums（枚举）

### 基本枚举

```rust
#[derive(Debug)]
enum Message {
    Resize,
    Move,
    Echo,
    ChangeColor,
    Quit,
}
// 使用：Message::Resize
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

**这些变体不是函数，是构造函数**。每次写 `Message::Move(Point { x: 10, y: 15 })` 就是在创建一个 `Message` 类型的值。

**核心价值**：同一种类型表达不同形态的数据，一个 `Vec<Message>` 就能装下各种形态的消息，然后用 `match` 分流处理——这是 Rust 版的**代数数据类型（ADT）/ 多态**，不用继承，不用接口。

### match 模式匹配

```rust
fn process(&mut self, message: Message) {
    match message {
        Message::Resize { width, height } => {
            self.resize(width, height);
        }
        Message::Move(p) => {
            self.move_position(p);
        }
        Message::Echo(s) => {
            self.echo(s);
        }
        Message::ChangeColor(r, g, b) => {
            self.change_color(r, g, b);
        }
        Message::Quit => {
            self.quit();
        }
    }
}
```

- **必须穷尽**（exhaustive）：漏掉任何一个变体，编译器都会报错
- **模式解构**：`Message::Resize { width, height }` 直接解出内部字段
- `message` 的所有权被移入 `match`，因为每个变体的值只使用一次，不需要保留

### 格式化输出

| 格式 | 用途 | 所需 trait | 获取方式 |
|---|---|---|---|
| `{}` | 用户友好输出 | `Display` | 手动实现 |
| `{:?}` | 调试输出 | `Debug` | `#[derive(Debug)]` 自动生成 |
| `{:#?}` | 美化调试输出 | `Debug` | 同上 |

---

## 速查表

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
| 迭代器 | `.iter().map(\|e\| e+1).collect()` | 函数式处理 |
| 经典结构体 | `struct S { x: u8 }` | 字段名访问 |
| 元组结构体 | `struct T(u8, u8)` | 下标访问 `.0` |
| 单元结构体 | `struct U;` | 无字段 |
| 结构体更新 | `S { x: 1, ..base }` | 继承其余字段 |
| 方法 | `fn m(&self) -> T { }` | 优先 &self |
| 关联函数 | `fn new() -> Self { }` | 类型名调用 |
| 枚举 | `enum E { A, B(u8) }` | 变体可带数据 |
| 模式匹配 | `match v { E::A => ..., E::B(x) => ... }` | 必须穷尽 |

---

## 9. Strings（字符串）

### String vs &str

Rust 有两种字符串类型：

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
let s3: &str = "字面量";        // 字面量本身就是 &str，编译期写死在二进制里
```

- `String` 是**所有者**，能改
- `&str` 是借来看的**视图**，不能改
- 函数参数**优先用 `&str`**，除非需要拿走所有权或修改内容

### 为什么 &str 不占堆？

字符串字面量在编译时就写死在二进制文件的只读数据区。`&str` 只是一个指向那片静态内存的 (指针 + 长度)，不需要堆分配。

### 动态大小类型（DST）

`str` 是 DST，大小编译期不知道，**不能直接当作变量类型**。Rust 中所有 DST 都必须藏在指针后面使用：

```rust
let s: &str = "hello";              // str 藏在 & 后面
let slice: &[i32] = &arr[0..3];     // [T] 藏在 & 后面
let trait_obj: &dyn ToString = &"hi"; // dyn Trait 藏在 & 后面

let n: i32 = 42;                    // i32 大小固定，可以直接用
let st: String = String::from("hi"); // String 大小固定 (24字节)
```

### 拥有 vs 借用对照表

```
String   : &str      // 拥有字符串  : 字符串切片
Vec<i32> : &[i32]    // 拥有动态数组 : 切片
PathBuf  : &Path     // 拥有路径    : 路径切片
```

核心规律：很多类型都是成对出现的——一个"拥有版"（堆上可变），一个"借用版"（轻量只读视图）。

### &[i32] 是什么

`&[i32]` 是 **i32 类型的切片引用**，和 `&str` 是同类概念（`&str` 本质是 `&[u8]` 加 UTF-8 保证）。

```rust
let arr: [i32; 5] = [1, 2, 3, 4, 5];
let slice: &[i32] = &arr[1..4];   // [2, 3, 4]
let whole: &[i32] = &arr;         // 整个数组借成切片

let v: Vec<i32> = vec![10, 20, 30];
let vs: &[i32] = &v;              // Vec 自动解引用成切片
```

### &str → String 四种写法

```rust
let s1 = "hello".to_owned();        // ToOwned trait — 意图最明确
let s2 = "hello".to_string();       // Display trait，内部调 to_owned()
let s3 = String::from("hello");     // From trait
let s4: String = "hello".into();    // Into trait，依赖类型推断
```

`to_owned()` 并非字符串专属。`&[i32]` 调 `to_owned()` 返回 `Vec<i32>`，`&Path` 调 `to_owned()` 返回 `PathBuf`——任何借用类型都可借此获得对应的拥有版本。

### String 的 + 运算符

```rust
let s1 = String::from("hello");
let s2 = String::from(" world");
let s3 = s1 + &s2;          // s1 被移动走了
// println!("{s1}");        // 编译错误！s1 已失效
```

`+` 的签名本质是 `fn add(self, rhs: &str) -> String`——左操作数被**消费**，右操作数是 `&str`。

**为什么 `&String` 能传给需要 `&str` 的位置？** —— **解引用强制转换（Deref Coercion）**：`&String` 自动变为 `&str`。

### 拼接方式对比

| 方式 | 消费左值 | 底层机制 |
|------|---------|---------|
| `s1 + &s2` | 是，s1 被移动 | 复用 s1 的堆缓冲区追加，少一次分配 |
| `s.push_str(&s2)` | 否（s 需 mut） | 在已有缓冲区末尾写入，可能触发扩容 |
| `format!("{s1}{s2}")` | 否 | 每次新建缓冲区，多一次分配 |

- 多段拼接**优先用 `format!`**，不拿所有权，可读性好
- 只想往已有 String 追加，用 `push_str` 或 `push`（单个字符）

### 练习要点

1. **`String::from("blue")`** — String 创建方式
2. **`word.as_str()`** — 从 `String` 变回 `&str`
3. **`input.trim()`** — 去除两端空格，返回 `&str`（不分配新内存）
4. **`input.replace("cars", "balloons")`** — 替换子串，返回新 `String`
5. **`format!("{} world!", input)`** — 格式化拼接，返回 `String`
6. **`&String::from("abc")[0..1]`** — 字符串切片，注意这是**字节索引**不是字符索引（中文字符一个占3字节，切到中间会 panic）。字符索引用 `s.chars().nth(INDEX)`

---

## 10. Modules（模块）

### 通俗理解

模块就是代码的"文件夹"——把相关代码组织在一起，形成命名空间。

### 三个核心关键字

| 关键字 | 作用 |
|--------|------|
| `mod` | 定义/声明一个模块 |
| `use` | 把模块路径引入当前作用域，简化访问 |
| `pub` | 让模块内的东西对外可见（默认一切都是私有的） |

### 基础用法

```rust
mod animal {
    fn secret() {}        // 默认私有，外部无法访问
    pub fn eat() {        // pub 才能被外部调用
        secret();
    }
}

fn main() {
    animal::eat();        // :: 路径分隔符
}
```

### 模块的文件组织方式

| 方式 | 写法 | 说明 |
|------|------|------|
| 内联 | `mod foo { ... }` | 模块内容直接写在同一个文件里 |
| 同目录文件 | `mod foo;` → `foo.rs` | 模块内容放在同名 `.rs` 文件 |
| 同目录文件夹 | `mod foo;` → `foo/mod.rs` | 模块内容放在同名文件夹的 `mod.rs` 里 |

### use 的常见写法

```rust
use std::collections::HashMap;      // 引入单个类型
use std::io::{self, Read, Write};   // 引入多个，self 指代 io 模块本身
use std::fs::*;                     // 引入所有公开项（谨慎使用）
```

### 可见性规则

- 不加 `pub` → 仅在当前模块及子模块可见
- `pub` → 外部可访问
- `pub(crate)` → 当前 crate 内可见
- `pub(super)` → 父模块可见
- `pub(in path)` → 指定路径内可见

### use 的可见性：默认私有

`use` 本身**不默认公开**——它只把名称引入当前模块作用域。想让外部也能通过本模块访问被引入的东西，需要加 `pub`：

```rust
mod a {
    mod b {
        pub fn hello() {}
    }
    use b::hello;      // 私有：a 内部能用，外部不能
    pub use b::hello;  // 重新导出（re-export）：外部可以通过 a::hello 调用
}
```

| 写法 | 当前模块可见 | 外部可见 |
|------|:----------:|:------:|
| `use xxx;` | ✓ | ✗ |
| `pub use xxx;` | ✓ | ✓ |

`pub use` 即 **re-export（重新导出）**——把内部模块里的东西以当前模块的名义"转发"出去。

### 与 Java 的类比

| Java | Rust | 作用 |
|------|------|------|
| `package com.example;` | `mod foo;` | 命名空间/组织代码 |
| `class Dog { ... }` | `struct Dog { ... }` + `impl Dog { ... }` | 数据 + 行为 |
| `import com.example.Dog;` | `use crate::foo::Dog;` | 引入路径 |

Rust 的 `mod` 更像 Java 的 `package`，只负责组织代码和控制可见性。数据和行为由 `struct` + `impl` 承担（对应 Java 的 `class`）。

### 练习要点

1. **`sausage_factory::make_sausage()`** — `pub fn` 才能跨模块调用，私有函数模块内调用不受限
2. **`pub use self::fruits::PEAR as fruit;`** — 用 `self` 指代当前模块，`as` 起别名，`pub use` 重新导出
3. **`use std::time::*;`** — 通配符引入 `SystemTime` 和 `UNIX_EPOCH`
