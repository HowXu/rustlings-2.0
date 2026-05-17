# Quizzes

After every couple of sections, there will be a quiz in this directory that'll test your knowledge on a bunch of sections at once.

---

## 笔记：Quiz 2 要点

### 1. `Vec<String>` 能和 `[&str; N]` 数组比较？

可以。因为标准库实现了 `Vec<T>: PartialEq<[U]>`（当 `T: PartialEq<U>` 时），而 `String` 实现了 `PartialEq<&str>`。所以 Rust 会逐元素把 `String` 和 `&str` 对齐比较：

```rust
let v: Vec<String> = vec!["hello".to_string()];
assert_eq!(v, ["hello"]);  // ✅ 编译通过
```

### 2. 为什么要用 `crate::` 引入 `my_module`？

`my_module` 定义在 crate 根层级（文件顶层），而 `tests` 是嵌套的子模块。从嵌套模块回到根层级：

```rust
mod tests {
    // 方式1: crate:: 从 crate 根出发
    use crate::my_module::transformer;

    // 方式2: super:: 往上一层
    use super::my_module::transformer;
}
```

`super::` = 上级模块，`crate::` = crate 根（绝对路径）。

### 3. 何时解引用？所有权怎么变？

**什么时候要 `*`：** 手上有引用 `&T`，但需要的是 `T` 本身的值。

**所有权规则：**
- `T: Copy`（如 i32, usize, bool）→ `*ref` 拷贝一份，原引用不受影响
- `T: !Copy`（如 String, Vec）→ **不能**通过引用移走所有权，必须 clone

```rust
// Copy 类型：解引用 = 拷贝
let x = 5;
let r = &x;
let y = *r;      // 拷贝 5，x 和 r 都还能用

// 非 Copy 类型：不能通过引用移走
let s = String::from("hi");
let r = &s;
// let t = *r;   // ❌ 编译错误
let t = r.clone(); // ✅ 必须 clone
```

### 4. `String` 和 `&str` 的自动转换（Deref 强制）

`String` 实现了 `Deref<Target=str>`，因此 `&String` 在需要 `&str` 的地方会自动转换：

| 代码 | `s` 实参类型 | 自动转换 |
|---|---|---|
| `s.to_uppercase()` | `&String` | `&String` → `&str`，调用 `str::to_uppercase()` |
| `s.trim()` | `&String` | `&String` → `&str`，调用 `str::trim()` |
| `r.push_str(s)` | `&String` | `push_str` 要 `&str`，自动转 |

**注意：** 只能 `&String → &str`，不能 `String → &str`。手上是 `String` 本身时，需要手动加 `&`。
