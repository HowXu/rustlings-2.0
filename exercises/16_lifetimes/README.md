# Lifetimes

Lifetimes tell the compiler how to check whether references live long
enough to be valid in any given situation. For example lifetimes say
"make sure parameter 'a' lives as long as parameter 'b' so that the return
value is valid".

They are only necessary on borrows, i.e. references,
since copied parameters or moves are owned in their scope and cannot
be referenced outside. Lifetimes mean that calling code of e.g. functions
can be checked to make sure their arguments are valid. Lifetimes are
restrictive of their callers.

If you'd like to learn more about lifetime annotations, the
[lifetimekata](https://tfpk.github.io/lifetimekata/) project
has a similar style of exercises to Rustlings, but is all about
learning to write lifetime annotations.

## Further information

- [Lifetimes (in Rust By Example)](https://doc.rust-lang.org/stable/rust-by-example/scope/lifetime.html)
- [Validating References with Lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)

---

## 笔记：生命周期语法

### 1. 标注语法：`'a`

生命周期是单引号开头的标识符，放在 `&` 和类型名之间：

```rust
&'a i32        // 带有生命周期 'a 的 i32 引用
&'a mut i32    // 可变引用同理
```

### 2. 函数签名

`<'a>` 放在函数名后（语法类似泛型），标注参数和返回值之间的关系：

```rust
// 编译器报错：不知道返回值跟哪个参数有关
fn longest(x: &str, y: &str) -> &str { ... }

// 正确：标注后编译器能验证
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

含义：x 和 y 至少活得跟 `'a` 一样久，返回值也活得跟 `'a` 一样久。

### 3. 结构体

```rust
struct Excerpt<'a> {
    part: &'a str,  // 引用必须比结构体实例活得久
}
```

### 4. 方法

```rust
impl<'a> Excerpt<'a> {
    fn announce_and_return(&self, announcement: &str) -> &str {
        println!("{}", announcement);
        self.part
    }
}
```

### 5. 生命周期省略规则（Elision）

编译器按三条规则自动补全，补不全才要求手写：

| 规则 | 说明 |
|------|------|
| 每个引用参数各自获得一个生命周期 | `fn foo(x: &str)` → `fn foo<'a>(x: &'a str)` |
| 只有一个输入生命周期时，返回值也用这个 | `fn foo(x: &str) -> &str` → `fn foo<'a>(x: &'a str) -> &'a str` |
| `&self` / `&mut self` 存在时，返回值用 self 的生命周期 | 所以方法经常不用手写生命周期 |

### 6. `'static`

程序整个运行期间都有效的引用：

```rust
let s: &'static str = "hello";  // 字符串字面量
```

### 7. 多个生命周期

```rust
fn foo<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    x  // 返回值只与 x 绑定
}
```

### 8. 命名规则

名字遵循标识符规则（字母、数字、下划线），大小写均可。约定使用单个小写字母 `'a`、`'b`……纯粹为了简洁。只有 `'static` 是保留关键字。

**核心理解：生命周期标注不改变代码逻辑，不延长任何变量的存活时间——只是给借用检查器提供信息，让它验证引用不会悬垂。**

---

## 笔记：lifetimes3 — 结构体中的生命周期

结构体字段持有引用时，必须标注生命周期：

```rust
struct Book<'a> {       // 'a 在结构体名后声明（类似泛型）
    author: &'a str,    // 这两个引用必须至少活得跟 'a 一样久
    title: &'a str,     // 'a 的具体长度由实例化时传入的引用决定
}
```

**为什么需要？** 没有 `'a`，编译器无法保证 `Book` 存活期间，它内部引用的数据没有被释放。标注后编译器就能追踪并验证。

### 补充：`println!` 会不会移动所有权？

不会。两个原因：
- `&str` 本身就是引用类型，传参只是复制引用，不涉及所有权。
- `println!` 宏内部用的是不可变借用（`&`），即使传 `String` 也不会拿走所有权：

```rust
let s = String::from("hello");
println!("{}", s);
println!("{}", s);  // 仍然可用
```
