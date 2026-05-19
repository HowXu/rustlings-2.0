# Clippy

The Clippy tool is a collection of lints to analyze your code so you can catch common mistakes and improve your Rust code.

If you used the installation script for Rustlings, Clippy should be already installed.
If not you can install it manually via `rustup component add clippy`.

## Further information

- [GitHub Repository](https://github.com/rust-lang/rust-clippy).

---

## 笔记

### 什么是 Clippy

Clippy 是 Rust 的官方 **lint 工具**，分析代码并给出改进建议。

### 常见 Clippy 建议

**1. 直接用常量，不要重复绑定**

```rust
// ❌ 多余的局部绑定
let pi = std::f32::consts::PI;

// ✅ 直接用
let area = std::f32::consts::PI * radius.powi(2);
```

**2. 单次模式匹配用 `if let`，不要用 `while let`**

```rust
// ❌ while let 暗示循环，实际只匹配一次
while let Some(x) = option { res += x; }

// ✅ if let 语义精确
if let Some(x) = option { res += x; }
```

**3. 用 `std::mem::swap` 代替手动交换**

```rust
// ❌ 手动三步交换
let temp = value_a;
value_a = value_b;
value_b = temp;

// ✅ 标准库一步到位
use std::mem::swap;
swap(&mut value_a, &mut value_b);
```

**4. 变量类型与命名要一致**

```rust
// ❌ 名叫 vec 却是单元类型
let my_empty_vec = ();
// ✅ 类型与名字一致
let my_empty_vec: Vec<i32> = vec![];
```

**5. 不要随意 `unwrap`**

```rust
// ❌ None 上 unwrap 会 panic
let my_option: Option<&str> = None;
// my_option.unwrap();  // 💥

// ✅ 用 if let 安全解构
if let Some(o) = my_option { println!("{}", o); }
```
