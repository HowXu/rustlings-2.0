# Tests

Going out of order from the book to cover tests -- many of the following exercises will ask you to make tests pass!

## Further information

- [Writing Tests](https://doc.rust-lang.org/book/ch11-01-writing-tests.html)

---

## 笔记

### `#[should_panic]` — 测试期望的 panic

有些函数的正确行为就是 panic（比如非法参数），测试时不能让它真的"炸掉"，而是告诉测试框架：**这个 panic 是预期的，panic 了才算通过**。

```rust
#[test]
#[should_panic]
fn negative_width() {
    Rectangle::new(-10, 10);  // 构造时 panic，测试通过
}
```

- 不加 `#[should_panic]` → panic 导致测试失败
- 加了 `#[should_panic]` → panic 即测试通过；没 panic 反而失败
- 常用于测试参数校验、边界条件等"必须拒绝"的情况

### 精准匹配 panic 消息（可选）

```rust
#[test]
#[should_panic(expected = "must be positive")]
fn negative_width() {
    Rectangle::new(-10, 10);
}
```

`expected` 参数限定 panic 消息必须**包含**指定字符串，防止"因其他原因 panic 却误报通过"。

### `#[...]` — 属性（Attribute）

属性是给编译器/工具链看的**元数据注解**，格式为 `#[...]`，只影响编译过程，不参与运行时逻辑。本质就是"给编译器传话"。

| 属性 | 作用 |
|------|------|
| `#[test]` | 标记函数为测试函数 |
| `#[should_panic]` | 测试应该 panic 才算通过 |
| `#[cfg(test)]` | 条件编译：只在测试模式编译 |
| `#[derive(Debug)]` | 自动生成 `Debug` trait 实现 |
| `#[allow(unused)]` | 抑制某个编译警告 |
| `#[warn(missing_docs)]` | 缺少文档时发出警告 |

**三种形式**：

```rust
#[test]                              // 无参数
#[should_panic(expected = "...")]    // 键值参数
#[cfg(any(unix, windows))]           // 布尔表达式参数
```

属性可以作用于不同目标：
- 函数、结构体、模块等（写在声明上方）
- crate 级别：`#![crate_type = "lib"]`（用 `!` 表示对整个 crate）
- 内部属性：`#![allow(unused)]` 写在模块开头，对整个模块生效
