# Options

Type Option represents an optional value: every Option is either Some and contains a value, or None, and does not.
Option types are very common in Rust code, as they have a number of uses:

- Initial values
- Return values for functions that are not defined over their entire input range (partial functions)
- Return value for otherwise reporting simple errors, where None is returned on error
- Optional struct fields
- Struct fields that can be loaned or "taken"
- Optional function arguments
- Nullable pointers
- Swapping things out of difficult situations

## Further Information

- [Option Enum Format](https://doc.rust-lang.org/book/ch10-01-syntax.html#in-enum-definitions)
- [Option Module Documentation](https://doc.rust-lang.org/std/option/)
- [Option Enum Documentation](https://doc.rust-lang.org/std/option/enum.Option.html)
- [if let](https://doc.rust-lang.org/rust-by-example/flow_control/if_let.html)
- [while let](https://doc.rust-lang.org/rust-by-example/flow_control/while_let.html)

---

## 笔记：`if let` 与 `unwrap` 的区别

`if let` 自带拆包能力，**不能**和 `.unwrap()` 混用：

```rust
let opt = Some(42);

// ✅ 正确：if let 自动取出值
if let Some(x) = opt {
    println!("{}", x);  // x = 42
}

// ❌ 错误：if let 和 unwrap 不能混
// if let x = opt.unwrap() { ... }

// ✅ unwrap 是另一种取法：None 时会 panic
let x = opt.unwrap();  // x = 42
let y: Option<i32> = None;
// y.unwrap();  // 💥 panic!
```

| | `if let` | `unwrap` |
|---|---|---|
| None 时 | 跳过不执行 | 程序崩溃（panic） |
| 做法 | `if let Some(x) = opt` | `opt.unwrap()` |
| 何时用 | 不确定是否有值 | 确定一定有值 |

---

## 笔记：`ref` 与 `&` 的区别

两者都跟引用有关，但位置和用法不同：

| | `&` | `ref` |
|---|---|---|
| 用在 | 表达式（值一侧） | 模式匹配（变量一侧） |
| 含义 | "创建这个值的引用" | "把匹配到的值绑定为引用" |

```rust
let x = 5;

// 两种写法等价：
let r = &x;        // & 在表达式侧，创建引用
let ref r = x;     // ref 在模式侧，声明绑定为引用

// match 中不移动所有权的两种写法：
let opt = Some(Point { x: 100, y: 200 });

// 写法1（传统）：ref 写在模式内
match opt {
    Some(ref p) => println!("{},{}", p.x, p.y),  // p: &Point
    _ => (),
}

// 写法2（现代）：& 加在 match 表达式上
match &opt {
    Some(p) => println!("{},{}", p.x, p.y),  // p: &Point（match ergonomics 自动转换）
    _ => (),
}
```

两者不能混用：`Some(ref p)` 不能写 `Some(&p)`，`match &opt` 不能写 `match ref opt`。


