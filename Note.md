# Rustlings 练习笔记 (Exercise 0-18)

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

## Rust 控制流（Control Flow）总览

### 1. if / else if / else — 条件分支

Rust 的 `if` 是**表达式**，可以直接用于赋值：

```rust
let x = 5;
let y = if x > 0 { 1 } else { -1 };   // 作为表达式赋值，所有分支必须返回同类型
```

**if 与 else 类型必须一致**：

```rust
let n = if true { 1 } else { 0 };    // ✅ 两边都是 i32
// let n = if true { 1 } else { "零" }; // ❌ 类型不匹配
```

**省略 else 时隐含返回 `()`**：

```rust
let x = 5;
let result = if x > 10 { "大" };     // ⚠️ else 被省略，隐含返回 ()
// result 的类型是 ()，不是 &str
```

**多条件链**：

```rust
let score = 85;
let grade = if score >= 90 { 'A' }
            else if score >= 80 { 'B' }
            else if score >= 70 { 'C' }
            else { 'F' };
```

### 2. loop — 无限循环

`loop` 创建一个无限循环，直到显式 `break`：

```rust
let mut count = 0;
loop {
    count += 1;
    if count == 10 {
        break;
    }
}
```

**loop 也是表达式 — `break` 可以带返回值**：

```rust
let mut counter = 0;
let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2;  // break 后面跟值，这就是整个 loop 的返回值
    }
};
println!("{}", result);  // 20
```

**循环标签（Label）** — 用于嵌套循环中指定 `break` / `continue` 的目标：

```rust
'outer: loop {
    println!("外层循环");
    loop {
        println!("  内层循环");
        break 'outer;    // 跳出外层循环，而不是只跳出内层
    }
}
// 执行结果：
// 外层循环
//   内层循环
// (结束)
```

```rust
let mut x = 0;
'outer: loop {
    x += 1;
    'inner: loop {
        if x == 5 { break 'outer; }   // 跳出外层
        if x == 3 { break 'inner; }   // 跳出内层
        break;                         // 跳出当前最近循环（等同于 break 'inner）
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
println!("发射!");
```

**while let** — 当模式匹配成功时持续循环：

```rust
let mut stack = vec![1, 2, 3];
while let Some(top) = stack.pop() {
    println!("{}", top);  // 依次打印 3, 2, 1
}
```

### 4. for — 迭代器循环（最惯用）

Rust 中 `for` 是最常用的循环方式，自动处理迭代器的边界安全：

```rust
let a = [10, 20, 30, 40, 50];

// 遍历元素（不可变引用）
for element in a.iter() {
    println!("值: {}", element);
}

// 遍历元素（可变引用）
let mut v = vec![1, 2, 3];
for element in v.iter_mut() {
    *element += 1;  // 修改原值
}

// 范围遍历
for number in 1..4 {     // 1..4 = [1, 2, 3] 左闭右开
    println!("{}!", number);
}

for number in 1..=4 {    // 1..=4 = [1, 2, 3, 4] 闭区间
    println!("{}!", number);
}

// 反向遍历
for number in (1..4).rev() {
    println!("{}!", number);  // 3, 2, 1
}
```

**`for` 等价形式**：`for x in iter` 等价于在迭代器上不断调 `.next()`，直到返回 `None`。编译器给数组和 Vec 做了特殊处理，`for x in arr` 等价于 `for x in arr.into_iter()`（消耗数组元素，但 i32 等 Copy 类型只复制）。

### 5. match — 模式匹配

Rust 的 `match` 类似于 C 的 `switch`，但强大得多——能匹配值、解构、绑定：

```rust
let x = 2;
match x {
    1 => println!("一"),
    2 => println!("二"),
    3 => println!("三"),
    _ => println!("其他"),  // _ 是通配符，匹配所有未列出的情况
}
```

**解构**：

```rust
enum Coin { Penny, Nickel, Dime, Quarter(u8) }  // Quarter 带一个 u8 数据

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(year) => {  // 解构出 year
            println!("{}年铸造", year);
            25
        }
    }
}
```

**`_` 通配符 vs 变量绑定**：

```rust
match some_value {
    1 => do_something(),
    _ => do_default(),    // _ 匹配一切，但不会绑定值
}

match some_value {
    1 => do_something(),
    other => {            // other 绑定匹配到的值，可以在分支中使用
        println!("值是: {}", other);
    }
}
```

**`|` 匹配多个值**：

```rust
let x = 1;
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

当你只关心一种匹配情况时，用 `if let` 比 `match` 更简洁：

```rust
let some_value = Some(3);

// match 写法
match some_value {
    Some(3) => println!("是三"),
    _ => (),
}

// if let 写法（等价上者，更简洁）
if let Some(3) = some_value {
    println!("是三");
}
```

**if let 带 else**：

```rust
if let Some(value) = optional {
    println!("有值: {}", value);
} else {
    println!("无值");
}
```

### 7. break 和 continue

| 关键字 | 作用 | 适用范围 |
|--------|------|---------|
| `break` | 立即退出当前循环 | `loop`, `while`, `for` |
| `continue` | 跳过本次迭代剩余代码，开始下一次循环 | `loop`, `while`, `for` |

`break` 在 `loop` 中可带返回值（见上文 loop 示例）。

### 8. 控制流对比表

| 场景 | 推荐写法 | 原因 |
|------|---------|------|
| 二选一分支 | `if / else` | 直接，语义明确 |
| 多分支根据值判断 | `match` | 穷尽检查，不会漏 |
| 只关心一种匹配 | `if let` | 比 match 少两行 |
| 无限循环 + 内部跳出 | `loop` | break 可带返回值 |
| 有明确终止条件 | `while` | 语义清晰 |
| 遍历集合/范围 | `for` | 最安全，无越界风险 |
| 遍历时修改元素 | `for ... in ... iter_mut()` | 标准做法 |
| 嵌套循环精确跳转 | 带标签的 `break 'label` | 避免歧义 |

### 9. 与 C/Java 的关键差异

| 概念 | C / Java | Rust |
|------|----------|------|
| `if` | 语句，不能赋值 | 表达式，可以赋值 |
| `switch` | 有 fall-through，需写 break | `match` 无 fall-through，自动穷尽检查 |
| 三元 `?:` | `a ? b : c` | 直接用 `if a { b } else { c }` |
| 数组越界 | 运行时不检查 | `for .. in` 迭代器自动避免越界 |
| C风格 `for` | `for (i=0; i<n; i++)` | 用 `for i in 0..n` |
| `while let` | 无 | 有，模式匹配 + 循环 |

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

---

## 11. Hashmaps（哈希映射）

### 基本概念

HashMap 就是键值对存储，C++ 叫 `unordered_map`，Python 叫 `dict`，其他语言称关联数组。

### 创建与插入

```rust
use std::collections::HashMap;

let mut basket = HashMap::new();
basket.insert(String::from("banana"), 2);
basket.insert(String::from("apple"), 3);
```

### `#[derive(Hash, PartialEq, Eq, Debug)]` 的含义

HashMap 的 key 必须同时满足 `Hash` + `Eq`（`Eq` 要求先有 `PartialEq`，所以实际上要三个）。

| trait | 作用 | 为什么需要 |
|-------|------|-----------|
| `Hash` | 把值计算成哈希值 | 通过哈希值快速定位 key |
| `PartialEq` | 允许 `==` 比较 | 哈希冲突时需要判断 key 是否真相等 |
| `Eq` | 标记 `==` 满足自反性（a == a 永远为真） | HashMap key 的强制要求 |
| `Debug` | 允许 `{:?}` 打印 | 方便调试（非必须） |

```rust
// 没有 derive — 不能用作 HashMap key
enum Fruit { Apple, Banana }
// map.insert(Fruit::Apple, 42);  // ❌ 编译错误

// 有 derive — 一切正常
#[derive(Hash, PartialEq, Eq, Debug)]
enum Fruit { Apple, Banana }
let mut map = HashMap::new();
map.insert(Fruit::Apple, 42);  // ✅
```

### entry API（最惯用写法）

`entry` 只做**一次**哈希查找，返回 `Entry` 枚举（`Vacant` 不存在 / `Occupied` 已存在），链式 API 丰富。

**只有 key 不存在时才会触发 `or_insert`**：

```rust
let mut map = HashMap::new();
map.insert("a", 1);

map.entry("a").or_insert(99);  // "a" 已存在，不动 → map["a"] 仍是 1
map.entry("b").or_insert(99);  // "b" 不存在，插入 → map["b"] 变成 99
```

### 三种插入方式对比

| 方式 | 推荐度 | 说明 |
|------|-------|------|
| `basket.entry(key).or_insert(val)` | 推荐 | 一次哈希查找，最惯用 |
| `basket.try_insert(key, val)` | 一般 | Rust 1.70+ 才稳定 |
| `if !contains_key { insert }` | 不推荐 | 两次哈希查找，啰嗦 |

### `and_modify` — 对已存在的值做修改

`and_modify` 只在 key **已存在**（`Occupied`）时触发，拿到 `&mut V` 让你修改原值。常与 `or_insert` 链式使用：

```rust
// 经典组合：计数
map.entry(key)
   .and_modify(|v| *v += 1)  // 已存在 → +1
   .or_insert(1);             // 不存在 → 插入 1

// 三种常见用法：
// 1. or_insert 返回 &mut，怎么改都行
let v = map.entry("a").or_insert(0);
*v += 10;  // 不管新旧，最终都 +10

// 2. 只改已存在的，不存在就跳过
map.entry("a").and_modify(|v| *v *= 2);

// 3. 不存在插入默认值，存在就修改
map.entry("a")
   .and_modify(|v| *v = v.to_uppercase())
   .or_insert("default".to_string());
```

执行逻辑：`Occupied` 走 `and_modify` 跳过 `or_insert`，`Vacant` 跳过 `and_modify` 走 `or_insert`。

### `HashMap::from_iter` — 从迭代器构建

```rust
let content = [(Fruit::Apple, 4), (Fruit::Mango, 2)];
let basket = HashMap::from_iter(content);
```

### `#[derive(Default)]`

自动实现 `Default` trait，让结构体字段全部取默认值（`u8` → `0`，`String` → `""` 等）。`or_insert` 配合 `Default` 无需手动写默认值。

### 练习要点

1. **`HashMap::new()`** — 创建空 HashMap，类型推断或手动标注
2. **`basket.entry(fruit).or_insert(3)`** — entry 占位，不存在就插入
3. **`entry().and_modify(...).or_insert(...)`** — 链式：存在就改，不存在就插

---

## 12. Options（可选值）

### 基本概念

`Option<T>` 表示一个可能存在也可能不存在的值。只有两个变体：`Some(T)` 和 `None`。

```rust
fn maybe_ice_cream(hour_of_day: u16) -> Option<u16> {
    if hour_of_day < 22 {
        Some(5)
    } else if hour_of_day <= 23 {
        Some(0)
    } else {
        None
    }
}
```

Rust 没有 `null`，用 `Option<T>` 替代。这迫使你在编译期就处理"值可能不存在"的情况。

### `if let` 与 `unwrap` 的区别

| | `if let` | `unwrap` |
|---|---|---|
| None 时 | 跳过不执行 | 程序崩溃（panic） |
| 写法 | `if let Some(x) = opt` | `opt.unwrap()` |
| 何时用 | 不确定是否有值 | 确定一定有值 |

```rust
let opt = Some(42);

// ✅ if let 自动取出值
if let Some(x) = opt {
    println!("{}", x);  // x = 42
}

// ✅ unwrap：确定有值时用
let x = opt.unwrap();  // x = 42

// ❌ if let 和 unwrap 不能混用
// if let x = opt.unwrap() { ... }
```

### `while let` — 循环版本

只要模式匹配就持续执行循环体：

```rust
let mut vec: Vec<Option<i8>> = vec![None, Some(1), Some(2), Some(3)];

while let Some(Some(integer)) = vec.pop() {
    println!("{}", integer);  // 依次输出 3, 2, 1
}
// vec.pop() 返回 Option<Option<i8>>
// 外层 Some 表示 pop 有值，内层 Some 取出实际数字
```

注意：`Vec::pop()` 返回 `Option<T>`，如果元素本身也是 `Option`，就会形成嵌套 `Option<Option<T>>`，需要**嵌套模式匹配**。

### `ref` 与 `&` 的区别

两者都跟引用有关，但位置和用法不同：

| | `&` | `ref` |
|---|---|---|
| 用在 | 表达式（值一侧） | 模式匹配（变量一侧） |
| 含义 | "创建这个值的引用" | "把匹配到的值绑定为引用" |

```rust
let x = 5;
let r = &x;        // & 在表达式侧，创建引用
let ref r = x;     // ref 在模式侧，声明绑定为引用

// match 中不移动所有权的两种写法：
let opt = Some(Point { x: 100, y: 200 });

// 写法1（传统）：ref 写在模式内
match opt {
    Some(ref p) => println!("{},{}", p.x, p.y),  // p: &Point
    _ => (),
}

// 写法2（现代，推荐）：& 加在 match 表达式上
match &opt {
    Some(p) => println!("{},{}", p.x, p.y),  // p: &Point（自动推断）
    _ => (),
}
```

现代 Rust 推荐写法2（`match &opt`），编译器会自动识别你在借用，模式中的绑定自动变成引用（这叫 **match ergonomics**）。

### 练习要点

1. **`Some(5)` / `None`** — Option 的两个变体
2. **`if let Some(x) = opt`** — 条件解构，None 时跳过
3. **`while let Some(x) = ...`** — 循环解构，遇 None 退出
4. **`match &opt`** — 借用匹配，不移动所有权

---

## 13. Error Handling（错误处理）

### 两类错误

Rust 把错误分成两类：
- **不可恢复**：`panic!`，直接崩溃
- **可恢复**：`Result<T, E>`，调用方决定怎么处理

### `Result<T, E>` 基础

```rust
enum Result<T, E> {
    Ok(T),    // 成功，携带返回值
    Err(E),   // 失败，携带错误信息
}

fn generate_nametag_text(name: String) -> Result<String, String> {
    if name.is_empty() {
        Err(String::from("Empty names aren't allowed"))
    } else {
        Ok(format!("Hi! My name is {name}"))
    }
}
```

### `?` 运算符

`?` 是 Rust 最常用的错误处理语法糖，本质是 `match` + `return Err` 的简写：

```rust
// 这两种写法完全等价：
let qty = item_quantity.parse::<i32>()?;

let qty = match item_quantity.parse::<i32>() {
    Ok(n) => n,                      // 成功 → 取出值
    Err(e) => return Err(e.into()),  // 失败 → 提前返回 Err
};
```

**使用条件**：`?` 只能在返回 `Result` 或 `Option` 的函数中使用，且错误类型必须兼容。

### `?` 在 `main` 中的使用

`main` 函数也可以返回 `Result`，从而在函数体内直接使用 `?`：

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cost = total_cost("8")?;     // ? 在 main 中直接用
    println!("You now have {} tokens.", 100 - cost);
    Ok(())
}
```

### `Box<dyn Error>` — 通用错误容器

当不同类型错误需要混用时，用 `Box<dyn Error>` 兜底。`dyn Error` 表示"任何实现了 `Error` trait 的类型"。

```rust
fn main() -> Result<(), Box<dyn Error>> {
    let x: i64 = "42".parse()?;                    // ParseIntError
    let num = PositiveNonzeroInteger::new(x)?;      // CreationError
    // 两种错误都能通过 ? 传播，因为都实现了 Error trait
    Ok(())
}
```

### `map_err` — 错误类型转换

`map_err` 只转换 `Err` 分支，`Ok` 原封不动：

```rust
// map_err 伪代码
fn map_err<T, E, F>(self, op: impl FnOnce(E) -> F) -> Result<T, F> {
    match self {
        Ok(val) => Ok(val),
        Err(e)  => Err(op(e)),
    }
}

// 实际用法
s.parse::<i64>()
    .map_err(ParsePosNonzeroError::ParseInt)?;  // ParseIntError → 自定义错误

Self::new(x)
    .map_err(ParsePosNonzeroError::from_creation)  // CreationError → 自定义错误
```

### 枚举变体直接当函数用

```rust
// ParsePosNonzeroError::ParseInt 本身就是一个 fn(ParseIntError) -> ParsePosNonzeroError
s.parse::<i64>().map_err(ParsePosNonzeroError::ParseInt)

// 等价于闭包写法：
s.parse::<i64>().map_err(|e| ParsePosNonzeroError::ParseInt(e))
```

### 自定义错误类型

```rust
enum ParsePosNonzeroError {
    Creation(CreationError),   // 业务错误
    ParseInt(ParseIntError),   // 解析错误
}

impl ParsePosNonzeroError {
    fn from_creation(err: CreationError) -> Self { Self::Creation(err) }
    fn from_parse_int(err: ParseIntError) -> Self { Self::ParseInt(err) }
}
```

### `impl Display for XxxError`

自定义错误要能被 `Box<dyn Error>` 接收，必须实现 `Display` + `Error` trait：

```rust
use std::fmt;

impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match self {
            CreationError::Negative => "number is negative",
            CreationError::Zero => "number is zero",
        };
        f.write_str(description)
    }
}

impl std::error::Error for CreationError {}
```

### 练习要点

1. **`Result<Ok, Err>`** — 替代 `Option` 提供错误信息
2. **`?` 运算符** — 成功取值，失败提前返回，等价于 `match` + `return Err`
3. **`main() -> Result<(), Box<dyn Error>>`** — 让 main 支持 `?`
4. **`Box<dyn Error>`** — 通用错误类型容器，抹平不同错误类型差异
5. **`map_err`** — 只转换 Err 分支，Ok 原封不动
6. **枚举变体直接当函数** — `map_err(MyError::Variant)` 等价于 `map_err(|e| MyError::Variant(e))`
7. **`impl Display + Error`** — 自定义错误要能被 `Box<dyn Error>` 接纳的必备条件

---

## 14. Generics（泛型）

### 基本概念

泛型就是**类型的占位符**，让同一份代码处理不同类型，减少重复。

### 泛型语法三要素

| 位置 | 写法 | 说明 |
|------|------|------|
| 结构体定义 | `struct Wrapper<T>` | `<T>` 声明在类型名后，T 可在结构体内使用 |
| 实现块 | `impl<T> Wrapper<T>` | `impl<T>` 声明泛型参数，`Wrapper<T>` 指定具体泛型 |
| 函数 | `fn foo<T>(x: T)` | 函数上同理 |

```rust
// 泛型结构体
struct Wrapper<T> {
    value: T,
}

// 泛型实现
impl<T> Wrapper<T> {
    fn new(value: T) -> Self {
        Wrapper { value }
    }
}

// 使用：编译器自动推断 T
let w1 = Wrapper::new(42);        // T = i32
let w2 = Wrapper::new("Foo");     // T = &str
```

### `Vec<T>` 的类型标注

`Vec::new()` 是泛型构造函数，没有上下文时编译器无法推断 `T`，需要手动标注：

```rust
let mut numbers: Vec<i16> = Vec::new();  // 显式标注 T = i16
```

### `Into` 与类型转换

`u8` 和 `i8` 都能 `.into()` 成 `i16`（因为 `i16` 实现了 `From<u8>` 和 `From<i8>`），所以 `Vec<i16>` 是能同时容纳这两种输入的合适类型。

### 泛型类比

```rust
// 没有泛型之前，每种类型都要单独写一遍
struct I32Wrapper { value: i32 }
struct StrWrapper { value: &'static str }

// 泛型之后，一份代码搞定所有类型
struct Wrapper<T> { value: T }
```

### 练习要点

1. **`Vec<i16>`** — 手动标注泛型类型帮助编译器推断
2. **`struct Wrapper<T>` + `impl<T> Wrapper<T>`** — 泛型结构体与实现的完整写法，`<T>` 出现两次（声明 + 绑定）

---

## 15. Traits（特征）

### 本质

Trait 定义一组方法签名，描述类型的**共享行为**。类似 Java interface / C++ 抽象类，但可以有**默认实现**。

### 基本语法

```rust
trait Licensed {
    // 必须实现的方法（无默认实现时）
    fn licensing_info(&self) -> String;
}

trait Licensed {
    // 带默认实现的方法（可选覆盖）
    fn licensing_info(&self) -> String {
        "Default license".to_string()
    }
}

struct SomeSoftware;
impl Licensed for SomeSoftware {}  // 空实现，使用默认方法
```

### `impl Type` vs `impl Trait for Type`

| 写法 | 含义 | 例子 |
|------|------|------|
| `impl Type { }` | 为类型添加**自身方法**（inherent） | `impl String { fn new() ... }` |
| `impl Trait for Type { }` | 让类型**实现某个 trait** | `impl Display for Point { fn fmt() ... }` |

关键差异：
1. 自身方法直接 `.` 调用；trait 方法需要 trait 在作用域内
2. 自身方法优先级 > trait 方法；同名冲突时用 `TraitName::method(&instance)` 调用 trait 版
3. 孤儿规则对 `impl Type` 更严格（只能在类型所在 crate 写）

### Trait 作为参数类型 — `impl Trait`

```rust
// 要求参数只要实现了 Licensed 即可，不限制具体类型
fn compare_license_types(software1: impl Licensed, software2: impl Licensed) -> bool {
    software1.licensing_info() == software2.licensing_info()
}
```

`impl Trait` 是语法糖，等价于泛型约束写法：
```rust
fn compare_license_types<T: Licensed, U: Licensed>(software1: T, software2: U) -> bool { ... }
```

### 多重 trait 约束 — `impl Trait1 + Trait2`

```rust
// 要求参数同时实现两个 trait
fn some_func(item: impl SomeTrait + OtherTrait) -> bool {
    item.some_function() && item.other_function()
}
```

泛型等价写法：
```rust
fn some_func<T: SomeTrait + OtherTrait>(item: T) -> bool { ... }
```

### 孤儿规则 (Orphan Rule)

**只能为"你的"trait 或"你的"类型写实现**。两边都外来的，编译器禁止：

| impl 组合 | Trait 归属 | Type 归属 | 是否允许 |
|-----------|-----------|----------|---------|
| `impl MyTrait for MyType` | 你的 | 你的 | ✅ |
| `impl MyTrait for String` | 你的 | std | ✅ trait 是你的 |
| `impl Display for MyType` | std | 你的 | ✅ type 是你的 |
| `impl Display for String` | std | std | ❌ 两边都外来 |

**破局 — Newtype 模式**：用元组结构体包装外来类型，再实现外来 trait：

```rust
struct MyStr(String);  // 你定义的包装类型
impl fmt::Display for MyStr {  // ✅ MyStr 是你的
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "【{}】", self.0)
    }
}
let inner: &String = &s.0;  // 通过 .0 访问内部值
```

### 常用标准库 Trait

| Trait | 用途 | 关联语法 |
|-------|------|---------|
| `Clone` | 显式复制 | `.clone()` |
| `Copy` | 隐式按位复制 | 赋值/传参时 |
| `Debug` | 调试打印 | `{:?}` |
| `Display` | 用户友好打印 | `{}` |
| `PartialEq` | 比较相等 | `==`, `!=` |
| `PartialOrd` | 比较大小 | `<`, `>`, `<=`, `>=` |
| `Default` | 提供默认值 | `T::default()` |
| `From` / `Into` | 类型转换 | `T::from(x)` / `x.into()` |
| `Drop` | 析构时自动调用 | 离开作用域时 |

### 练习要点

1. **`fn append_bar(self) -> Self`** — trait 方法中 `Self` 指代实现者类型，`self` 拿走所有权
2. **`impl AppendBar for Vec<String>`** — 为泛型类型的特定具体化实现 trait
3. **默认方法** — trait 方法带函数体即可，实现者不写也能直接用
4. **`impl Licensed` 做参数** — 接受任何实现了 Licensed 的类型，类似接口多态
5. **`impl A + B`** — `+` 组合多个 trait 约束

---

## 16. Lifetimes（生命周期）

### 核心概念

生命周期是编译器验证**引用是否有效**的机制。它不改变代码逻辑，不延长变量存活时间——只给借用检查器提供信息，确保引用不会悬垂。

### 语法：`'a`

生命周期标注是单引号开头的标识符，放在 `&` 和类型名之间：

```rust
&'a i32        // 带有生命周期 'a 的 i32 引用
&'a mut i32    // 可变引用同理
```

### 函数中的生命周期

当函数返回引用，且编译器无法推断返回值与哪个参数相关时，需要标注：

```rust
// ❌ 编译器报错：不知道返回值跟 x 还是 y 有关
fn longest(x: &str, y: &str) -> &str { ... }

// ✅ 标注后编译器能验证
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

含义：x 和 y 至少活得跟 `'a` 一样久，返回值也活得跟 `'a` 一样久。

### 结构体中的生命周期

```rust
struct Book<'a> {
    author: &'a str,   // 引用必须比结构体实例活得久
    title: &'a str,
}
```

没有 `'a`，编译器无法保证 `Book` 存活期间内部引用的数据不会被释放。

### 生命周期省略规则（Elision）

编译器按三条规则自动补全，补不全才要求手写：

| 规则 | 说明 |
|------|------|
| 每个引用参数各自获得一个生命周期 | `fn foo(x: &str)` → `fn foo<'a>(x: &'a str)` |
| 只有一个输入生命周期时，返回值也用这个 | `fn foo(x: &str) -> &str` → `fn foo<'a>(x: &'a str) -> &'a str` |
| `&self` / `&mut self` 存在时，返回值用 self 的生命周期 | 所以方法经常不用手写生命周期 |

### `'static` 生命周期

程序整个运行期间都有效的引用：

```rust
let s: &'static str = "hello";  // 字符串字面量天生是 'static
```

### 多个生命周期

```rust
fn foo<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    x  // 返回值只与 x 绑定
}
```

### lifetimes2 关键教训

```rust
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(&string1, &string2);  // string2 在这个作用域内
    }  // ← string2 在这里被销毁
    println!("The longest string is '{result}'");  // ❌ result 引用了已销毁的 string2！
}
```

**生命周期以作用域为单位**。内层花括号定义的变量，其生命周期短于外层变量。即使 `longest` 的返回值实际上指向 `string1`，编译器仍按最坏情况来检查——因为 `'a` 同时约束了 `string1` 和 `string2`，取两者的**交集**（较短的那个），`result` 的引用效期不能超过 `string2` 的生命周期。

### 命名规则

名字遵循标识符规则（字母、数字、下划线），约定使用单个小写字母 `'a`、`'b`……只有 `'static` 是保留关键字。

### `println!` 会不会移动所有权？

不会。`&str` 本身就是引用，传参只是复制引用。即使传 `String`，`println!` 宏内部用不可变借用，不会拿走所有权：

```rust
let s = String::from("hello");
println!("{}", s);
println!("{}", s);  // 仍然可用
```

### 练习要点

1. **`fn longest<'a>(x: &'a str, y: &'a str) -> &'a str`** — 输入与输出的生命周期关系
2. **`struct Book<'a> { author: &'a str }`** — 结构体持有引用必须标注生命周期
3. **生命周期取交集** — 多个 `'a` 约束取最短的那个，内层作用域的引用活不到外层

---

## 17. Tests（测试）

### 测试模块结构

```rust
#[cfg(test)]           // 只在测试模式编译，不进入正式构建
mod tests {
    use super::*;     // super 指向上层模块，导入待测试的函数

    #[test]
    fn test_something() { ... }
}
```

`super` 关键词：在子模块中引用父模块，`super::` 就是"上一层"。

### 三种断言宏

| 宏 | 用途 | 示例 |
|----|------|------|
| `assert!` | 断言布尔值为 true | `assert!(is_even(2));` |
| `assert_eq!` | 断言两值相等 | `assert_eq!(power_of_2(2), 4);` |
| `assert_ne!` | 断言两值不等 | `assert_ne!(x, y);` |

### `#[should_panic]`

有些函数的正确行为就是 panic（如非法参数）——测试时告诉框架"这个 panic 是预期的"：

```rust
#[test]
#[should_panic]
fn negative_width() {
    Rectangle::new(-10, 10);  // panic 发生 → 测试通过
}
```

- 不加 `#[should_panic]` → panic 导致测试失败
- 加了 `#[should_panic]` → panic 即测试通过；没 panic 反而失败
- 精准匹配：`#[should_panic(expected = "must be positive")]` 限制 panic 消息必须包含指定字符串

### `#[...]` — 属性（Attribute）

属性是给编译器/工具链看的**元数据注解**，格式 `#[...]`，只影响编译过程，不参与运行时：

| 属性 | 作用 |
|------|------|
| `#[test]` | 标记函数为测试函数 |
| `#[should_panic]` | 测试应该 panic 才算通过 |
| `#[cfg(test)]` | 条件编译：只在测试模式编译 |
| `#[derive(Debug)]` | 自动生成 `Debug` trait 实现 |
| `#[allow(unused)]` | 抑制某个编译警告 |

**三种形式**：

```rust
#[test]                              // 无参数
#[should_panic(expected = "...")]    // 键值参数
#[cfg(any(unix, windows))]           // 布尔表达式参数
```

### 练习要点

1. **`use super::*;`** — 从父模块导入待测函数，`super` 指上一层
2. **`assert!` / `assert_eq!`** — 两种最常用的断言
3. **`#[should_panic]`** — 测试 panic 行为，不加期望消息可能因其他原因 panic 误报通过

---

## 18. Iterators（迭代器）

### 三种迭代方式

| 方法 | 产出类型 | 原集合状态 | 使用场景 |
|------|----------|------------|----------|
| `.iter()` | `Option<&T>` | 仍然可用 | 只读遍历 |
| `.iter_mut()` | `Option<&mut T>` | 仍然可用 | 修改元素 |
| `.into_iter()` | `Option<T>` | 被消耗掉 | 拿走所有权 |

```rust
let arr = ["a", "b", "c"];  // [&str; 3]

// .iter() → 每个元素是 &T，返回 &&str
let mut it = arr.iter();
assert_eq!(it.next(), Some(&"a"));
// arr 还能继续用 ✅

// .into_iter() → 拿走所有权，返回 T 本身（&str）
let mut it2 = arr.into_iter();
assert_eq!(it2.next(), Some("a"));
// println!("{:?}", arr);  // ❌ arr 已被消耗
```

### `.next()` — 迭代器核心方法

`next()` 返回 `Option<T>`，有值返回 `Some`，耗尽返回 `None`。迭代器是**惰性**的（lazy），不调消费方法不会执行。

### `.collect()` — 万能收集器

`.collect()` 根据返回类型自动将迭代器收集为目标集合：

```rust
// 收集为 Vec<String>
words.iter().map(|s| capitalize_first(s)).collect()  // → Vec<String>

// 收集为 String
words.iter().map(|s| capitalize_first(s)).collect()  // → String

// Result 短路收集：全 Ok → Ok(vec)，遇 Err → 返回该 Err
numbers.into_iter().map(|n| divide(n, 27)).collect()  // → Result<Vec<i64>, _>

// Result 原样收集：每个 Result 独立保留
numbers.into_iter().map(|n| divide(n, 27)).collect()  // → Vec<Result<i64, _>>
```

**同一个 `.collect()` 只因目标类型不同就产生完全不同的行为**——`Result` 版本是标准库为 `FromIterator` 做的特殊实现。

### `.chain()` — 拼接迭代器

把两个同类型迭代器串成一个，懒求值：

```rust
first.to_uppercase()       // ToUppercase 迭代器（一个字符可能变大写后变多个字符）
     .chain(chars)           // 把剩余字符接在后面
     .collect::<String>()    // 收集为字符串
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

| 方法 | 闭包返回 | 行为 | 适用场景 |
|------|----------|------|----------|
| `filter(f)` | `bool` | `true` 保留原元素，`false` 丢弃 | 纯过滤 |
| `filter_map(f)` | `Option<T>` | `Some(x)` 保留 `x`，`None` 丢弃 | 过滤 + 转换 |

```rust
// filter：只判断条件
nums.iter().filter(|x| *x % 2 == 0).collect()  // → [2, 4]

// filter_map：同时过滤和转换
nums.iter().filter_map(|x| if *x % 2 == 0 { Some(x * 2) } else { None }).collect()  // → [4, 8]

// ❌ 注意：filter_map 总是返回 Some → 等于没过滤
map.iter().filter_map(|(_,v)| Some(*v == value)).count()  // 错误！count 了全部条目
```

### `flat_map` — 摊平嵌套迭代器

```rust
// collection: &[HashMap<_, _>]
// flat_map 把每个 HashMap 的值迭代器摊平成一个平层迭代器
collection.iter().flat_map(|m| m.values()).filter(|v| *v == value).count()
```

### `values()` vs `iter()` on HashMap

```
&HashMap<K, V>  →  .values()  →  Iterator<Item = &V>        // 直接拿值
&HashMap<K, V>  →  .iter()    →  Iterator<Item = (&K, &V)>   // 拿键值对
```

比 `map.iter()` 再解构元组更简洁。

### 练习要点

1. **`arr.iter()`** — 借出迭代，返回 `&T`，原集合仍可用
2. **`.next()`** — 惰性消费，返回 `Option<&T>`
3. **`.collect()`** — 目标类型决定行为，Same method, different outcome
4. **`first.to_uppercase().chain(chars).collect()`** — `to_uppercase()` 返回迭代器，用 `chain` 拼接
5. **`.product()` / `.sum()`** — 无循环求积/求和，空迭代器返回单位元（1/0）
6. **`.filter()`** — 返回 bool，true 保留；`filter_map` 返回 Option，同时过滤+转换
7. **`.flat_map()`** — 摊平嵌套迭代器

---

## 17. Tests（测试）

### 测试宏

| 宏 | 用途 |
|------|------|
| `assert!(条件)` | 断言条件为真 |
| `assert_eq!(a, b)` | 断言 a == b |
| `assert_ne!(a, b)` | 断言 a != b |

### 测试 panic

```rust
#[test]
#[should_panic]
fn test_negative_width() {
    Rectangle::new(-10, 10);  // 期望它 panic
}
```

`#[should_panic]` 告诉测试框架：这个函数应该 panic，panic 了才算通过。

### 测试模块结构

```rust
#[cfg(test)]
mod tests {
    use super::*;  // 引入外层模块的所有内容

    #[test]
    fn my_test() {
        // ...
    }
}
```

- `#[cfg(test)]` — 测试代码只在 `cargo test` 时编译
- `use super::*` — 测试模块是外层模块的子模块，需要导入要测试的函数/结构体
