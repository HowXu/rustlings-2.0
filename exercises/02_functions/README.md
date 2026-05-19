# Functions

Here, you'll learn how to write functions and how the Rust compiler can help you debug errors even
in more complex code.

## Further information

- [How Functions Work](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)

---

## 笔记

### 函数签名与返回

- 参数必须标注类型：`fn call_me(num: i64)`
- 返回值用 `->` 标注：`fn square(num: i32) -> i32`
- 函数签名不加 `mut`，因为 `mut` 不是类型的一部分（`fn foo(mut x: i32)` 只在函数内可改局部副本）

### 语句 vs 表达式

- 带 `;` 是语句（statement），不返回值
- 不带 `;` 是表达式（expression），会作为返回值
- 函数体最后一个表达式就是返回值（无需 `return` 关键字）

```rust
fn square(num: i32) -> i32 {
    num * num   // 无 ; → 表达式，作为返回值
}
```

### 闭包（Closure）基础

```rust
|参数1, 参数2| { 函数体 }      // 完整写法
|a, b| a + b                  // 单表达式，省略花括号
|| println!("hi")             // 无参数
```

闭包与函数的本质区别：闭包**能捕获外部环境变量**。
