# Vectors

Vectors are one of the most-used Rust data structures. In other programming
languages, they'd simply be called Arrays, but since Rust operates on a
bit of a lower level, an array in Rust is stored on the stack (meaning it
can't grow or shrink, and the size needs to be known at compile time),
and a Vector is stored in the heap (where these restrictions do not apply).

Rust中，数组存储在堆栈上(这意味着不能增大或缩小，并且大小需要在编译时知道)，
向量存储在堆中（堆不受这些限制）。

Vectors are a bit of a later chapter in the book, but we think that they're
useful enough to talk about them a bit earlier. We shall be talking about
the other useful data structure, hash maps, later.

## Further information

- [Storing Lists of Values with Vectors](https://doc.rust-lang.org/book/ch08-01-vectors.html)
- [`iter_mut`](https://doc.rust-lang.org/std/primitive.slice.html#method.iter_mut)
- [`map`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map)

---

## 笔记：vec! 宏的正确用法

**错误写法：**

```rust
let mut result = vec![&str];  // ❌ 这是把类型名当元素，编译不过
```

`vec!` 宏里要放的是**具体的值**，不是类型名。

**正确写法：**

```rust
// 空 Vec，需要显式标注类型
let mut result: Vec<&str> = vec![];
let mut result: Vec<&str> = Vec::new();

// 有初始值的 Vec
let mut result = vec!["hello", "world"];  // 类型自动推断为 Vec<&str>
```
