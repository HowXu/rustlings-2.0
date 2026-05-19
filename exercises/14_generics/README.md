# Generics

Generics is the topic of generalizing types and functionalities to broader cases.
This is extremely useful for reducing code duplication in many ways, but can call for some rather involved syntax.
Namely, being generic requires taking great care to specify over which types a generic type is actually considered valid.
The simplest and most common use of generics is for type parameters.

## Further information

- [Generic Data Types](https://doc.rust-lang.org/book/ch10-01-syntax.html)
- [Bounds](https://doc.rust-lang.org/rust-by-example/generics/bounds.html)

---

## 笔记

### 基本概念

泛型就是**类型的占位符**，让同一份代码处理不同类型，减少重复。

### 泛型语法三要素

| 位置 | 写法 | 说明 |
|------|------|------|
| 结构体定义 | `struct Wrapper<T>` | `<T>` 声明在类型名后 |
| 实现块 | `impl<T> Wrapper<T>` | `impl<T>` 声明参数，`Wrapper<T>` 绑定到类型 |
| 函数 | `fn foo<T>(x: T)` | 同理 |

```rust
struct Wrapper<T> {
    value: T,
}

impl<T> Wrapper<T> {
    fn new(value: T) -> Self {
        Wrapper { value }
    }
}

let w1 = Wrapper::new(42);    // T = i32（编译器推断）
let w2 = Wrapper::new("Foo"); // T = &str
```

### `Vec<T>` 的类型标注

空 `Vec::new()` 没有上下文时编译器无法推断 `T`，需手动标注：

```rust
let mut numbers: Vec<i16> = Vec::new();  // 显式标注 T = i16
```

### 泛型类比

```rust
// 没有泛型：每种类型写一遍
struct I32Wrapper { value: i32 }
struct StrWrapper { value: &'static str }

// 有泛型：一份代码搞定
struct Wrapper<T> { value: T }
```
