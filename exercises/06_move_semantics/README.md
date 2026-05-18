# Move Semantics

These exercises are adapted from [pnkfelix](https://github.com/pnkfelix)'s [Rust Tutorial](https://pnkfelix.github.io/rust-examples-icfp2014/) -- Thank you Felix!!!

## Further information


## Note
所有权 借用和引用
移动语义

赋值、传参、返回值时，所有权会转移，原变量失效。

核心规则：一个值同时只能有一个所有者。当把值赋给另一个变量时，所有权就"移动"过去了。

For this section, the book links are especially important.

- [Ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
- [Reference and borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)


```rs
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;  // s1 的所有权"移动"到了 s2

    // println!("{}", s1);  // 编译错误！s1 已经失效
    println!("{}", s2);     // 正确，s2 才是现在的所有者

    take_ownership(s2);
    // println!("{}", s2);  // 编译错误！s2 也失效了，所有权移进了函数
}

fn take_ownership(s: String) {
    println!("被拿走的: {}", s);
}  // s 被 drop

// 所以你这样显式传入不被自动复制的参数会导致上下文失效
// 如果要实现传入函数再正常返回应该链式调用显式return

fn take_ownership(s: String) -> String {
    println!("被拿走的: {}", s);
    s
}

// 当然 这种情况引用更好
```

实现了 Copy trait 的类型（如 i32、bool）不会移动，而是自动复制，原变量还能用。

```rs
let x = 5;
let y = x;    // 复制，不是移动
println!("{}", x);  // 没问题
```

复杂类型（String、Vec 等堆上数据）默认移动；简单标量默认复制。想用克隆就显式 .clone()。


mut 本质上就是一个权限标记：

```rs
let     a = vec![1, 2, 3];  // a 有所有权，权限：只读
let mut b = vec![1, 2, 3];  // b 有所有权，权限：读写
let     r = &a;              // r 借 a，权限：只读
let     rm = &mut b;         // rm 借 b，权限：读写
```

你可以把它看作两维权限系统：

|       |    所有权    |     只读借用     |   读写借用 |
|:-------:|--------------|------------------|------------|
|绑定:  | let a = v    |   let r = &v     |let rm = &mut v|
|权限:  | 读+释放      |   只读           |    读+写   |
|写法:  | 不需 mut     |   不需 mut       | 需要 mut (在v)|
mut 的含义取决于它出现的位置：

```rs
let mut v = vec![1];  // 给 v 这个绑定加可写权限
&mut v                // 请求一个带写权限的借用
fn f(x: &mut T)       // 函数索要带写权限的借用

```
所以 Rust 所有关于"能不能改"的问题，归根结底就是一句话：你当前通过哪个名字、以什么权限访问那个地址。mut 就是这个权限的通行证。

Vec.push的方法签名
```rs
impl<T> Vec<T> {
    pub fn push(&mut self, value: T) { ... }
}
```
v.push(4)  →  方法调用自动取引用  →  &mut v  →  传给 push(&mut self, ...)

. 操作符会自动为你取 & 或 &mut，但它不能无中生有——&mut 只能从 mut 绑定生成。let v 不写 mut，就是声明"这个绑定不给 &mut 权限"，那后面所有需要 &mut 的操作都走不通。

### 字段访问不会移走所有权

```rust
struct Rectangle {
    width: i32,
    height: i32,
}

let rect = Rectangle::new(10, 20); // rect 拥有整个结构体
assert_eq!(rect.width, 10);        // 仅读取字段，rect 所有权不变
assert_eq!(rect.height, 20);       // 同上，rect 仍然有效
```

字段访问（`rect.width`）是**读取/借用**，不会把 `rect` 的所有权交出去。理由：
- 字段访问只读取数据，不是移动整个结构体
- `i32` 实现了 `Copy` trait，即使"移动"也会自动复制
- `assert_eq!` 宏内部取的是 `&` 引用，不拿所有权

**对比**：

| 操作 | 所有权变化 |
|------|-----------|
| `rect.width` 字段访问 | 不变（读取） |
| `&rect` 借用 | 不变（共享借用） |
| `let rect2 = rect;` 赋值 | 转移，rect 失效 |
| `fn consume(r: Rectangle)` 传参 | 转移，实参失效 |
| 实现了 `Copy` 的类型（如 `i32`） | 自动复制，不变 |