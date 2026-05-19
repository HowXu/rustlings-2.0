# Enums

Rust allows you to define types called "enums" which enumerate possible values.
Enums are a feature in many languages, but their capabilities differ in each language. Rust's enums are most similar to algebraic data types in functional languages, such as F#, OCaml, and Haskell.
Useful in combination with enums is Rust's "pattern matching" facility, which makes it easy to run different code for different values of an enumeration.

## Further information

- [Enums](https://doc.rust-lang.org/book/ch06-00-enums.html)
- [Pattern syntax](https://doc.rust-lang.org/book/ch19-03-pattern-syntax.html)


|格式|	用途	|实现 trait	|必须 derive?
|---|---|---|---|
|{}	|用户友好输出|	Display	|手动实现
|{:?}	|调试输出	|Debug	|#[derive(Debug)] 自动生成
|{:#?}|	美化 Debug|	Debug	|同上

---

## 笔记

### Match Guard（守卫）

**语法**：`模式 if 条件`，模式匹配通过后再检查条件，两关都过才执行分支。Guard 失败不会报错，会继续尝试下一分支。

```rust
match value {
    模式 if 条件 => 表达式,
}
```

#### 基础用法

```rust
let num = 4;
match num {
    x if x < 0       => println!("负数"),
    x if x % 2 == 0  => println!("正偶数"),
    x                => println!("正奇数"),
}
```

#### 与解构结合（核心优势）

Guard 能访问**解构后绑定的变量**，这是 `if/else` 做不到的：

```rust
let point = (3, 10);

match point {
    (x, y) if x == y        => println!("在对角线上"),
    (x, y) if x > 5 && y > 5 => println!("都在右上角"),
    (x, _) if x < 0         => println!("在 y 轴左边"),
    _                       => println!("其他位置"),
}
```

枚举解构 + guard：

```rust
enum Shape {
    Circle(f64),           // 半径
    Rectangle(f64, f64),   // 宽, 高
}

let s = Shape::Rectangle(3.0, 10.0);
match s {
    Shape::Circle(r) if r > 100.0       => println!("大圆"),
    Shape::Rectangle(w, h) if w == h    => println!("正方形"),
    Shape::Rectangle(w, h) if w * h > 50.0 => println!("大面积矩形"),
    _ => println!("其他"),
}
```

#### Guard 调用方法

```rust
match s.trim() {
    name if name.is_empty() => println!("空字符串"),
    name if name.len() > 10 => println!("太长"),
    name                    => println!("名字：{name}"),
}
```

#### 与 `|` 或模式交互

```rust
match num {
    2 | 4 | 6 if num > 3 => println!("大于3的偶数"),  // 4 或 6
    2 | 4 | 6           => println!("<=3的偶数"),     // 2
    _                   => println!("其他"),
}
```

`if` 作用于整个 `|` 组合，无法拆分给各个分支。

#### 对比

| 特性 | match guard | if/else |
|------|-------------|---------|
| 模式解构 | ✅ 内置 | ❌ 需先 `let` 解构 |
| 穷尽性检查 | ✅ 编译器强制 | ❌ 可能遗漏 |
| 适合场景 | 复杂模式 + 条件 | 纯条件判断 |