# Macros

Rust's macro system is very powerful, but also kind of difficult to wrap your
head around. We're not going to teach you how to write your own fully-featured
macros. Instead, we'll show you how to use and create them.

If you'd like to learn more about writing your own macros, the
[macrokata](https://github.com/tfpk/macrokata) project has a similar style
of exercises to Rustlings, but is all about learning to write Macros.

## Further information

- [The Rust Book - Macros](https://doc.rust-lang.org/book/ch20-05-macros.html)
- [The Little Book of Rust Macros](https://veykril.github.io/tlborm/)
- [Rust by Example - macro_rules!](https://doc.rust-lang.org/rust-by-example/macros.html)

---

## 学习笔记

### 宏是什么？

宏是**写代码的代码**——在编译时把一段简写展开成完整的 Rust 代码。函数名后面没有 `!`，宏后面一定带 `!`。

### 基本语法

```rust
macro_rules! 宏名 {
    (模式) => { 展开体 };
    (另一个模式) => { 另一个展开体 };
}
```

- `macro_rules!` + 宏名称（不用 `!`）
- `{ }` 内是匹配规则，每条 `(模式) => { 展开体 }`，用 `;` 分隔
- 模式写在 `( )` 里，类似 `match` 的匹配臂

### 最简单示例

```rust
macro_rules! hello {
    () => {
        println!("Hello!");
    };
}

fn main() {
    hello!(); // 展开后就是 println!("Hello!");
}
```

### 带参数的宏

```rust
macro_rules! say {
    ($msg:expr) => {
        println!("你说：{}", $msg);
    };
}
```

- `$msg:expr` —— `$` 开头是**元变量**，`:expr` 是**片段分类符**

### 支持多个参数

```rust
macro_rules! add {
    ($a:expr, $b:expr) => {
        $a + $b
    };
}

let result = add!(3, 5); // 展开为 3 + 5
```

### 多条匹配规则（重载）

```rust
macro_rules! vector {
    () => { Vec::new() };
    ($elem:expr) => { vec![$elem] };
    ($($elem:expr),+ $(,)?) => { vec![$($elem),+] };
}
```

这就是 `vec!` 的工作原理——根据调用形式匹配不同规则。

### 重复匹配

```rust
macro_rules! sum {
    ($($x:expr),+) => {{
        let mut total = 0;
        $(total += $x;)+  // 对每个匹配的 $x 重复这段代码
        total
    }};
}
```

**重复运算符**：

| 符号 | 含义 |
|------|------|
| `*` | 零次或多次 |
| `+` | 一次或多次 |
| `?` | 零次或一次 |

### 常用片段分类符

| 分类符 | 匹配内容 |
|--------|----------|
| `expr` | 表达式 |
| `ident` | 标识符（变量名、函数名） |
| `ty` | 类型 |
| `tt` | 单个 token 树（最灵活） |
| `literal` | 字面量（`42`, `"hi"`） |
| `stmt` | 语句 |
| `pat` | 模式 |

### 宏 vs 函数

| | 函数 | 宏 |
|---|---|---|
| 参数数量 | 固定 | 可变（`+` / `*`） |
| 参数类型 | 固定类型 | 任意匹配 |
| 可变参数 | 不支持 | 天然支持 |
| 调用方式 | `fn()` | `macro!()` |
| 展开时机 | 运行时 | 编译时 |

### 使用场景

1. 避免重复代码——多处相似逻辑，泛型搞不定时
2. `println!` 这类——需要编译时检查格式字符串
3. DSL（领域特定语言）——如 `lazy_static!`、`html!` 等库

### 实用例子：自动实现 Debug

```rust
macro_rules! debug_struct {
    ($name:ident { $($field:ident),* $(,)? }) => {
        #[derive(Debug)]
        struct $name {
            $($field: i32,)*
        }
    };
}

debug_struct!(Point { x, y });
// 展开为：
// #[derive(Debug)]
// struct Point { x: i32, y: i32 }
```

### 宏的作用域（macros3 核心）

宏有**自己的作用域规则**：定义在模块里的宏，默认是私有的，外面看不到。和函数不同——函数可以用 `模块名::函数名` 访问，宏不行。

**三种解决办法：**

```rust
// 方法一：#[macro_use] 放 module 上（一口气导出模块内所有宏）
#[macro_use]
mod macros {
    macro_rules! my_macro {
        () => { println!("Check out my macro!"); };
    }
}

// 方法二：use 导入（精确，Rust 2018+ 推荐）
mod macros {
    macro_rules! my_macro { ... };
}
fn main() {
    use macros::my_macro;
    my_macro!();
}

// 方法三：#[macro_export] 标记导出（跨 crate 也能用，库开发常用）
mod macros {
    #[macro_export]
    macro_rules! my_macro { ... };
}
```

| 方法 | 写法 | 特点 |
|------|------|------|
| `#[macro_use]` | 放 module 上 | 一口气导出模块内所有宏 |
| `use` | 放函数体内或文件顶部 | 精确导入，更推荐 |
| `#[macro_export]` | 放宏定义上 | 跨 crate 也能用，常用在库中 |

**关键点：** 宏定义必须在使用之前（macros2），且必须在调用者的可见作用域内（macros3）。
