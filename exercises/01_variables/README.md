# Variables

In Rust, variables are immutable by default.
When a variable is immutable, once a value is bound to a name, you can't change that value.
You can make them mutable by adding `mut` in front of the variable name.

## Further information

- [Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)

函数参数上加 mut 和引用 &mut 是两回事，容易搞混。
```rs
// 1. 参数上 mut: 只是允许在函数内部改这个局部绑定
fn foo(mut x: i32) {
    x = x + 1;       // mut 允许重新赋值
    println!("{}", x); // 10
}
// 调用方不受影响，x 是复制进去的

// 2. &mut 引用: 借来改原值
fn bar(x: &mut i32) {
    *x += 1;          // 通过引用改原值
}

fn main() {
    let a = 5;
    foo(a);           // a 复制进去，a 还是 5

    let mut b = 5;
    bar(&mut b);      // 传入可变引用
    println!("{}", b); // 6，原值被改了
}
```

本质区别：

fn f(mut x: T)	fn f(x: &mut T)
改的是	函数内的局部副本	调用方原来的值
调用方写法	f(val)	f(&mut val)
调用方变量	不需要 mut	必须是 let mut
不带 mut 的参数也是不可变的，和 let 一样：

```rs
fn foo(x: i32) {
    // x = 6;  // 编译错误！参数默认也不可变
}
```

所以规则是统一的：所有绑定默认不可变，想改就加 mut，let 如此，参数也如此。参数上的 mut 只是让你在函数里能改这个局部变量，跟调用方无关。

let 默认不可变的不是"这个值永远不能改"，而是**"这条绑定不能改"**

但同一个值可以通过不同路径访问，不同路径可以有不同权限。

```rs
fn main() {
    let x = 5;              // 这条绑定 x 不可变
    // x = 6;               // 错误！不能通过 x 改

    let r = &x;             // r 是不可变引用：不能通过 r 改
    // *r = 6;              // 错误！r 没有改的权限

    let mut y = 5;          // 这条绑定 y 可变
    y = 6;                  // 行，通过 y 改

    let rm = &mut y;        // rm 有可变引用：授权通过 rm 改
    *rm = 7;                // 行，通过 rm 改

    // 但！有 rm 在的期间，y 自己不许碰
    // let z = y;           // 错误！y 的可变引用还在外面
}

```

所以是三层独立性：

```
值的可变性 → 值自己有 &mut 时就能改，不管 let 是否加 mut
绑定的可变性 → let mut 才能重新赋值这个变量名
引用的可变性 → &mut 给你"可以改"的令牌，& 只给"看"的令牌
```

打个比方：let 是存折上的名字不能涂改；&mut 是有人把存折借你让你取钱；值本身改没改跟存折名字能不能改是两码事。

这恰恰是 Rust 安全的基础：你可以用不可变绑定 let x 拥有一个 Vec，同时用 &mut x 借给别人改，完事后再自己用——所有权清晰，不会出数据竞争。如果 let 不可变就意味着值永远不能改，那 Rust 就写不了任何有用的程序了。

let其实是象征地址与变量的绑定 我们可以获得只读或者可改可读的变量

```rs
fn main() {
    let x = 10;
    // x  是名字
    // 10 是值，放在栈上某个地址
    // 绑定：x → 这个地址
    // 权限：只读

    let mut y = 10;
    // y → 另一个地址
    // 权限：可读可写

    let z = y;    // 复制了值，绑定到新地址
    let r = &y;   // r 绑到 y 的地址，权限只读
    let rm = &mut y; // rm 也绑到 y 的地址，权限可写
    
    // 稍微考虑一下这是实现了Copy Trail的变量类型 如果是Vec类型
    // let z = y;    // 移动语义了
    // let r = &y;   // 这个绑不了
}
```