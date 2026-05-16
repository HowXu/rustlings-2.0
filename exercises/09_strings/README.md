# Strings

Rust has two string types, a string slice (`&str`) and an owned string (`String`).
We're not going to dictate when you should use which one, but we'll show you how
to identify and create them, as well as use them.

## Further information

- [Strings](https://doc.rust-lang.org/book/ch08-02-strings.html)

string slice (`&str`) 此为何物
an owned string (`String`) owned是什么意思

```rs
fn main() {
    // String: 拥有堆上数据，可增删改
    let mut s1 = String::from("hello");
    s1.push_str(" world");         // 能改
    s1.push('!');                  // 能追加
    println!("{s1}");              // hello world!

    // &str: 借用，只读视图，不拥有数据
    let s2: &str = &s1;           // 从 String 借
    let s3: &str = "字面量";        // 字面量本身就是 &str，编译期写死在二进制里
    // s2.push_str(...)            // 错误！&str 不可变

    // 切片
    let slice: &str = &s1[0..5];  // "hello"
    println!("{slice}");
}
```

||	String|	&str|
|-----|-----|-----|
数据在哪|	堆，自己管理|	不拥有，指别人的数据或静态区
可变	|是|	否
开销	|栈上 24 字节(ptr+len+cap)	|栈上 16 字节(ptr+len)
传参	|移动所有权|	不移动，轻量借用
创建	|String::from to_string()|	"字面量" &s[..]

String 是所有者，能改；&str 是借来看的"视图"，不能改。函数参数优先用 &str，除非你需要拿走所有权或修改内容。

因为字符串字面量在编译时就写死在二进制文件的只读数据区了，&str 只是一个指向那片静态内存的指针+长度，不需要堆分配。

i32 是固定大小类型（4字节），可以直接放栈上。str 是动态大小类型（DST），大小编译期不知道，不能直接当作变量类型。

```rs
fn main() {
    // str — 字符串切片
    let s: &str = "hello";

    // [T] — 泛型切片
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let slice: &[i32] = &arr[0..3];  // 必须 &[T]

    // dyn Trait — trait 对象
    fn print_len(x: &dyn ToString) {
        println!("{}", x.to_string());
    }
    let use_it: &dyn ToString = &"hi";
    print_len(use_it);

    // 对比：固定大小类型可以直接用
    let n: i32 = 42;          // 行
    let st: String = String::from("hi");  // 行，String 大小固定(24字节)
}
```

# String 的加法（+ 运算符）

Rust 的 `String` 支持 `+` 运算符，通过 `std::ops::Add` trait 实现：

```rust
// + 的实际签名为：fn add(self, rhs: &str) -> String
// 左操作数是 self（消费所有权），右操作数是 &str

let s1 = String::from("hello");
let s2 = String::from(" world");
let s3 = s1 + &s2;          // s1 被移动走了，&s2 是借用
// println!("{s1}");        // 编译错误！s1 已失效
println!("{s3}");            // "hello world"
```

**多次拼接对比：**

```rust
let a = String::from("hello");
let b = String::from(" ");
let c = String::from("world");

// + 链式：丑陋，每次中间结果都被消费
let result = a + &b + &c;

// format!：干净，不拿所有权，可读性好
let a = String::from("hello");
let result = format!("{a}{b}{c}");  // a,b,c 都还在
```

| 方式 | 消费左值 | 底层机制 |
|------|---------|---------|
| `s1 + &s2` | 是，s1 被移动 | 复用 s1 的堆缓冲区追加，少一次分配 |
| `s.push_str(&s2)` | 否（s 需 mut） | 在已有缓冲区末尾写入，可能触发扩容 |
| `format!("{s1}{s2}")` | 否 | 每次新建缓冲区，多一次分配 |

**总结：**
- `+` 一次只能拼接一个 `&str`，且拿走左值所有权
- 多段拼接优先用 `format!`
- 只想往已有 String 追加，用 `push_str` 或 `push`（单个字符）
- 为什么右值能传 `&String`？因为**解引用强制转换（Deref Coercion）**：`&String` 自动变为 `&str`

# to_owned() 是什么意思

`to_owned()` = 把**借来的**数据变成**自己拥有的**。来自 `ToOwned` trait：

```rust
pub trait ToOwned {
    type Owned;
    fn to_owned(&self) -> Self::Owned;
}

// 对于 &str：type Owned = String
let s: &str = "rust is fun!";       // 借用，指向静态区
let owned: String = s.to_owned();   // 拥有，堆上新分配
```

**四种 `&str → String` 写法，本质相同：**

```rust
let s1 = "hello".to_owned();        // ToOwned trait — 意图最明确
let s2 = "hello".to_string();       // Display trait，内部调 to_owned()
let s3 = String::from("hello");     // From trait
let s4: String = "hello".into();    // Into trait，依赖类型推断
```

**注意：** `to_owned()` 并非字符串专属。`&[i32]` 调 `to_owned()` 返回 `Vec<i32>`，`&Path` 调 `to_owned()` 返回 `PathBuf`。它的核心理念是：**任何借用类型都可借此获得对应的拥有版本**。

# &[i32] 是什么

`&[i32]` 是 **i32 类型的切片引用**，和 `&str` 是同类概念（`&str` 本质是 `&[u8]` 加 UTF-8 保证）。

```rust
// 从固定数组创建切片
let arr: [i32; 5] = [1, 2, 3, 4, 5];
let slice: &[i32] = &arr[1..4];   // [2, 3, 4]
let whole: &[i32] = &arr;         // 整个数组借成切片

// 从 Vec 创建切片
let v: Vec<i32> = vec![10, 20, 30];
let vs: &[i32] = &v;              // Vec 自动解引用成切片
```

**内存布局：**

| 类型 | 栈上大小 | 说明 |
|------|---------|------|
| `i32` | 4 字节 | 直接存值 |
| `[i32; 5]` | 20 字节 | 5个i32依次排列在栈上 |
| `Vec<i32>` | 24 字节 | (ptr, len, cap)，数据在堆上 |
| `&[i32]` | 16 字节 | (ptr, len)，不拥有数据 |
| `&str` | 16 字节 | (ptr, len)，不拥有数据 |

**类比关系（拥有 vs 借用）：**

```
String   : &str      // 拥有字符串  : 字符串切片
Vec<i32> : &[i32]    // 拥有动态数组 : 切片
PathBuf  : &Path     // 拥有路径    : 路径切片
```

核心规律：Rust 中很多类型都是成对出现的——一个"拥有版"（堆上可变），一个"借用版"（轻量只读视图）。