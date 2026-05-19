# Type conversions

Rust offers a multitude of ways to convert a value of a given type into another type.

The simplest form of type conversion is a type cast expression. It is denoted with the binary operator `as`. For instance, `println!("{}", 1 + 1.0);` would not compile, since `1` is an integer while `1.0` is a float. However, `println!("{}", 1 as f32 + 1.0)` should compile. The exercise [`using_as`](using_as.rs) tries to cover this.

Rust also offers traits that facilitate type conversions upon implementation. These traits can be found under the [`convert`](https://doc.rust-lang.org/std/convert/index.html) module.
The traits are the following:

- `From` and `Into` covered in [`from_into`](from_into.rs)
- `TryFrom` and `TryInto` covered in [`try_from_into`](try_from_into.rs)
- `AsRef` and `AsMut` covered in [`as_ref_mut`](as_ref_mut.rs)

Furthermore, the `std::str` module offers a trait called [`FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html) which helps with converting strings into target types via the `parse` method on strings. If properly implemented for a given type `Person`, then `let p: Person = "Mark,20".parse().unwrap()` should both compile and run without panicking.

These should be the main ways ***within the standard library*** to convert data into your desired types.

## Further information

These are not directly covered in the book, but the standard library has a great documentation for it.

- [conversions](https://doc.rust-lang.org/std/convert/index.html)
- [`FromStr` trait](https://doc.rust-lang.org/std/str/trait.FromStr.html)

---

## 笔记

### Turbofish 语法 `::<>`

`::<>` 用于**调用泛型方法时**显式指定类型参数，避免编译器把 `<` 误解为比较运算符。

```rust
// 类型声明：尖括号直接跟类型名
let v: Vec<u8> = Vec::new();           // Vec<u8> 是类型名，不需要 ::

// 泛型方法调用：需要 ::<> 消歧义
let n = "42".parse::<u8>().unwrap();   // ✅ turbofish
// let n = "42".parse<u8>().unwrap();  // ❌ 编译器困惑：parse < u8？

// 等价替代：类型标注交给变量
let n: u8 = "42".parse().unwrap();     // ✅ 类型从变量推断
```

对比：

| 形式 | 含义 | 示例 |
|------|------|------|
| `Type<u8>` | 类型声明，带泛型参数 | `Vec<u8>` |
| `func::<u8>()` | 泛型函数/方法调用，显式指定类型 | `parse::<u8>()` |

### 关于所有权：`u8` 不会丢失所有权

你的代码：

```rust
let mut age: u8 = 0;
if let Ok(_age) = strs[1].parse::<u8>() {
    age = _age;  // 没事，完全安全
}
```

`age = _age` 不会导致 `age` 丢失所有权，因为 `u8` 实现了 **`Copy` trait**。

Rust 中赋值分两种情况：

| 类型 | 赋值行为 | 赋值后原变量 |
|------|----------|------------|
| `Copy` 类型（`u8`, `i32`, `bool`, `f64` 等） | 按位复制 | 仍可用 |
| 非 `Copy` 类型（`String`, `Vec` 等） | 所有权转移 | 不可再用 |

```rust
let a: u8 = 42;
let b = a;       // u8 是 Copy，a 的值被复制给 b
println!("{a}"); // ✅ a 还能用

let s = String::from("hello");
let t = s;       // String 不是 Copy，所有权转移给 t
// println!("{s}"); // ❌ s 已失效
```

所有基础数值类型和 `bool` 都是 `Copy`，所以你的 `age = _age` 只是把值拷了一份，完全没问题。

### `AsRef<T>` 和 `AsMut<T>` — 零开销引用转换

**核心思想**：不获取所有权，不拷贝数据，只借出一个引用。和 `From`/`Into` 的根本区别：

| 维度 | `From` / `Into` | `AsRef` / `AsMut` |
|------|----------------|-------------------|
| 所有权 | 消耗或新建数据 | 不消耗，只借用 |
| 开销 | 可能有 | **零开销** |
| 返回 | `Self` | `&T` 或 `&mut T` |

#### AsRef — 不可变借用转换为 `&T`

```rust
// String 实现 AsRef<str> → 零开销借出 &str
let s = String::from("hello");
let r: &str = s.as_ref();  // 指向 s 内部那片堆内存，没有新分配

// 泛型妙用：一个函数通吃 &str 和 String
fn print_len<T: AsRef<str>>(s: T) {
    println!("{}", s.as_ref().len());
}
print_len("hello");             // &str ✅
print_len(String::from("hi"));  // String ✅
```

常见实现：

| 类型 | 实现 | 借出 |
|------|------|------|
| `String` | `AsRef<str>` | `&str` |
| `&str` | `AsRef<str>` | `&str` |
| `Vec<u8>` | `AsRef<[u8]>` | `&[u8]` |
| `PathBuf` | `AsRef<Path>` | `&Path` |

#### AsMut — 可变借用转换为 `&mut T`

```rust
let mut v = vec![1, 2, 3];
let s: &mut [u32] = v.as_mut();
s[0] = 99;  // 通过可变引用修改内部数据
```

#### `&` vs `as_ref()` — 区别

`&` 是语言内置取引用运算符，`as_ref()` 是 `AsRef` trait 的方法：

```rust
let s = String::from("hello");

let r1: &String = &s;        // & 只是给 String 加引用，类型还是 &String
let r2: &str    = s.as_ref(); // as_ref() 穿透包装，直接借出内部 &str
```

| | `&` | `as_ref()` |
|------|-----|-----------|
| 本质 | 语言运算符 | trait 方法 |
| 类型 | 不会变 `T → &T` | 可以变 `String → &str` |
| 泛型 | 不灵活，只能取原类型引用 | 多态，同一个调用适配多种类型 |

比喻：
- `&` = 给快递盒拍照，拿到的还是盒子
- `as_ref()` = 拆开快递盒看里面，不管盒子是 `String` 还是 `&str`，统一拿到 `&str`

#### 一揽子实现（Blanket Impl）：`&mut T` 怎么就能调 `as_mut()`

标准库给 `&mut T` 写了这样一条规则（伪代码）：

```rust
// 只要 T 实现了 AsMut<U>，&mut T 就自动也能 AsMut<U>
impl<T: AsMut<U>, U> AsMut<U> for &mut T {
    fn as_mut(&mut self) -> &mut U {
        T::as_mut(self)  // &mut T 转交给底层的 T 去实现
    }
}
```

**调用链路解析**（以 `Box<u32>` 为例）：

```
fn num_sq<T: AsMut<u32>>(arg: &mut T) {
    arg.as_mut()
}

调用 num_sq(&mut Box::new(3)) 时：
  T = Box<u32>，arg 的类型是 &mut Box<u32>

1. &mut Box<u32> 有 AsMut<u32> 吗？
   → 有！一揽子实现：Box<u32> 自己有 AsMut<u32>，所以 &mut Box<u32> 自动获得

2. &mut Box<u32>::as_mut 内部调用 Box::<u32>::as_mut(self)
   → Box::as_mut 通过 DerefMut 穿透盒子，返回 &mut u32
```

**核心**：`arg.as_mut()` 不是"引用的引用再解引用"，而是靠 **trait 的一揽子实现** 把 `&mut T` 的调用转发给 `T` 自己去解。一步就能穿透引用 + 包装两层。`AsRef` 同理。`&mut u32` 直接实现 `AsMut<u32>`，所以链不长直接拿到。`&mut Box<u32>` 靠转发绕一圈到 Box 的 DerefMut。

#### 为什么 `as_mut(&mut self)` 参数必须是 `&mut self`

因为它返回的是 `&mut T`——指向**原数据内部**的引用，原数据必须活着。

```rust
// ✅ 正确设计：拿着原数据借出引用
trait AsMut<T> { fn as_mut(&mut self) -> &mut T; }

// ❌ 如果写成 self：函数结束 self 销毁，返回的引用成了悬垂指针
// fn as_mut(self) -> &mut T;  // Rust 编译器绝不允许
```

对比其他 trait 就清楚了：

| trait | 参数 | 所有权 | 返回 |
|-------|------|--------|------|
| `Into<T>` | `self` | 拿走所有权 | `T`（新值，独立存活） |
| `AsRef<T>` | `&self` | 只读借用 | `&T`（共享引用） |
| `AsMut<T>` | `&mut self` | 独占借用 | `&mut T`（可变引用） |

**为什么 `as_mut` 必须是 `&mut self` 而不能是 `&self`？**

要返回 `&mut T`，前提是调用者能**独占访问内部数据**。`&self` 只给只读权，无法保证没有别人也在看同一块内存，那就不能安全地借出可变引用。

```
as_ref(): &self  → &T     // 只读借只读，共享引用没问题
as_mut(): &mut self → &mut T  // 可变借可变，必须独占，&self 做不到
```

Rust 借用规则：同一时刻只能有一个 `&mut`。如果传 `&self` 给你，可能还有别的 `&self` 在外面飘着，你再返回 `&mut` 就违反规则了。
