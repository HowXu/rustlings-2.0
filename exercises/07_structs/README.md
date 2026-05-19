# Structs

Rust has three struct types: a classic C struct, a tuple struct, and a unit struct.

## Further information

- [Structures](https://doc.rust-lang.org/book/ch05-01-defining-structs.html)
- [Method Syntax](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)

---

## 笔记

### 三种结构体

```rust
// 经典结构体（类似 C/JSON）
struct ColorRegularStruct {
    red: u8,
    green: u8,
    blue: u8,
}

// 元组结构体（用下标访问）
struct ColorTupleStruct(u8, u8, u8);

// 单元结构体（无字段，值就是名字本身）
#[derive(Debug)]
struct UnitStruct;
```

### 实例化与访问

```rust
// 经典结构体 — 字段名:值 写法
let green = ColorRegularStruct { red: 0, green: 255, blue: 0 };

// 元组结构体 — 按位置传值
let green = ColorTupleStruct(0, 255, 0);

// 单元结构体 — 值就是名字
let unit = UnitStruct;

// 访问字段：经典用 .field_name，元组用 .0、.1、.2
```

### 结构体更新语法

```rust
let your_order = Order {
    name: String::from("Hacker in Rust"),
    count: 1,
    ..order_template   // 其余字段从 order_template 继承
};
```

### 方法（impl）

```rust
impl Package {
    // 关联函数（静态方法）：无 &self，用 Self 返回类型
    fn new(sender_country: String, weight_in_grams: u32) -> Self {
        Self { sender_country, weight_in_grams }
    }

    // 实例方法：&self 借用实例（优先用 &self 而非拿走所有权）
    fn is_international(&self) -> bool {
        self.sender_country != self.recipient_country
    }

    fn get_fees(&self, cents_per_gram: u32) -> u32 {
        cents_per_gram * self.weight_in_grams
    }
}
```

- **无 `self` 参数** = 关联函数（静态方法），调用用 `Package::new(...)`
- **`&self` 参数** = 实例方法，调用用 `package.method()`
- **`Self`** 关键字指代 `impl` 所在的类型名
- **优先用 `&self`**：借用不会触发所有权转移
