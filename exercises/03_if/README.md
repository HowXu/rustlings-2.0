# If

`if`, the most basic (but still surprisingly versatile!) type of control flow, is what you'll learn here.

## Further information

- [Control Flow - if expressions](https://doc.rust-lang.org/book/ch03-05-control-flow.html#if-expressions)

---

## 笔记

### `if` 是表达式

Rust 中 `if` 可以直接用于赋值（C/Java 需要三元运算符 `?:`）：

```rust
let identifier = if animal == "crab" {
    1
} else if animal == "gopher" {
    2
} else {
    0
};  // 注意这里需要 ;
```

### 关键规则

- **所有分支必须返回相同类型**
- 如果只有 `if` 没有 `else`，省略的 else 隐含返回 `()`（单元类型）
- 和 C 不同：条件不需要括号，但必须是 `bool` 类型（不支持 `if 1` 这种隐式转换）
