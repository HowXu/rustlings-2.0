# Rustlings 练习笔记 (Exercise 0-6)

> 提取自 `exercises/00_intro` ~ `exercises/06_move_semantics` 的注释与文档。

---

## 0. Intro（入门）

### 宏（Macros）

Rust 使用 `print!` 和 `println!` 宏向控制台输出文本。`println!` 语句末尾可以不加分号 `;`（因为返回 `()`，而函数期望 `()`）。

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
// 只借不看 → 传引用
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// 需要所有权 → 传值（但本例中 to_uppercase 返回新 String，可以只用引用）
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
