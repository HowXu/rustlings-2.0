# Traits

A trait is a collection of methods.

Data types can implement traits. To do so, the methods making up the trait are defined for the data type. For example, the `String` data type implements the `From<&str>` trait. This allows a user to write `String::from("hello")`.

In this way, traits are somewhat similar to Java interfaces and C++ abstract classes.

Some additional common Rust traits include:

- `Clone` (the `clone` method)
- `Display` (which allows formatted display via `{}`)
- `Debug` (which allows formatted display via `{:?}`)

Because traits indicate shared behavior between data types, they are useful when writing generics.

## Further information

- [Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)

---

## 笔记

### Trait 本质
- 定义一组方法签名，描述类型的**共享行为**
- 类似 Java interface / C++ 抽象类，但可以有**默认实现**

### 基本语法
```rust
trait Greet {
    fn greet(&self) -> String;           // 必须实现的方法
    fn greet_loud(&self) -> String {     // 有默认实现的方法（可选覆盖）
        self.greet().to_uppercase()
    }
}

struct Cat;
impl Greet for Cat {
    fn greet(&self) -> String { "meow".into() }
}

fn main() {
    let c = Cat;
    println!("{}", c.greet());       // meow
    println!("{}", c.greet_loud());  // MEOW（使用了默认实现）
}
```

### 孤儿规则 (Orphan Rule) — 详解

**规则**：你必须拥有 trait 或类型中的至少一方，才能写 `impl Trait for Type`。两边都是外来的，一律禁止。

**动机**：防止项目引入的两个不同库对同一个 `(Trait, Type)` 组合提供了冲突的实现，导致编译器无法抉择。

#### 允许与否一览（假设你在写自己的 crate）

| impl 组合 | Trait 归属 | Type 归属 | 是否允许 |
|-----------|-----------|----------|---------|
| `impl MyTrait for MyType` | 你的 | 你的 | ✅ |
| `impl MyTrait for String` | 你的 | std | ✅ trait 是你的 |
| `impl Display for MyType` | std | 你的 | ✅ type 是你的 |
| `impl Display for String` | std | std | ❌ 两边都外来 |
| `impl Display for Vec<T>` | std | std | ❌ 同上 |

#### 破局：Newtype 模式

想为外来类型实现外来 trait 时，用元组结构体包装一层：

```rust
use std::fmt;

struct MyStr(String);  // 你定义的包装类型

impl fmt::Display for MyStr {  // ✅ 合法：MyStr 是你的
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "【{}】", self.0)
    }
}

let s = MyStr("hello".into());
println!("{}", s);  // 输出：【hello】
let inner: &String = &s.0;  // 通过 .0 访问内部值
```

可实现 `Deref<Target = String>` 让 `MyStr` 自动获得 `String` 的所有方法。

#### 另一个坑：同一组合只能实现一次

即便你拥有 type，也不能为它实现已经被某个依赖实现过的 `(Trait + Type)` 组合——重复实现直接编译报错。

### 常用标准库 Trait
| Trait | 用途 | 关联语法 |
|-------|------|---------|
| `Clone` | 显式复制 | `.clone()` |
| `Copy` | 隐式按位复制（标记 trait） | 赋值/传参时 |
| `Debug` | 调试打印 | `{:?}` |
| `Display` | 用户友好打印 | `{}` |
| `PartialEq` | 比较相等 | `==`, `!=` |
| `PartialOrd` | 比较大小 | `<`, `>`, `<=`, `>=` |
| `Drop` | 析构时自动调用 | 离开作用域时 |
| `Default` | 提供默认值 | `T::default()` |
| `From` / `Into` | 类型转换 | `T::from(x)` / `x.into()` |

### Trait 作为泛型约束
```rust
fn say_hello<T: Greet>(x: T) {
    println!("{}", x.greet());
}
// 等价写法：
fn say_hello(x: impl Greet) {
    println!("{}", x.greet());
}
```

### #[derive] 自动实现
`#[derive(Debug, Clone, PartialEq)]` 可以让编译器自动为结构体/枚举生成对应的 trait 实现，前提是结构体的所有字段也实现了这些 trait。

---

## Q&A: `impl Type` vs `impl Trait for Type`

| 写法 | 含义 | 例子 |
|------|------|------|
| `impl Type { }` | 为类型添加**自身方法**（inherent） | `impl String { fn new() ... }` |
| `impl Trait for Type { }` | 让类型**实现某个 trait** | `impl Display for Point { fn fmt() ... }` |

### 差异对比
1. **调用**：自身方法直接 `.` 调用；trait 方法需要 trait 在作用域内
2. **孤儿规则**：`impl Type` 只能在类型所在 crate 写；`impl Trait for Type` 宽限到 trait 或类型任一方所在 crate
3. **同名冲突**：自身方法优先级 > trait 方法；想调 trait 的方法用 `TraitName::method(&instance)`

### 代码示例
```rust
struct Dog;

// ① 自身方法 — 直接挂在类型上
impl Dog {
    fn bark(&self) { println!("汪汪"); }
}

// ② 实现 trait — 满足某种特质契约
impl std::fmt::Display for Dog {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "一只狗")
    }
}

let d = Dog;
d.bark();          // ① 自身方法，随时可用
println!("{}", d); // ② Display trait 方法，被 println! 调用
```
