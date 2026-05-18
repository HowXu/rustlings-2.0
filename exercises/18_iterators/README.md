# Iterators

This section will teach you about Iterators.

## Further information

- [Iterator](https://doc.rust-lang.org/book/ch13-02-iterators.html)
- [Iterator documentation](https://doc.rust-lang.org/stable/std/iter/)

---

## 笔记

### Q: 为什么 `.iter().next()` 返回 `Some(&"banana")` 而不是 `Some("banana")`？

**核心原因**：`.iter()` 借出（borrow）元素，而不是拿走（move）元素，所以返回的是引用 `&T`。

三种迭代方式对比：

| 方法 | 产出类型 | 原集合状态 | 使用场景 |
|------|----------|------------|----------|
| `.iter()` | `Option<&T>` | 仍然可用 | 只读遍历 |
| `.iter_mut()` | `Option<&mut T>` | 仍然可用 | 修改元素 |
| `.into_iter()` | `Option<T>` | 被消耗掉 | 拿走所有权 |

```rust
let arr = ["a", "b", "c"];  // [&str; 3]

// .iter() → 每个元素是 &&str
let mut it = arr.iter();
assert_eq!(it.next(), Some(&"a"));  // Some(&&str)
assert_eq!(it.next(), Some(&"b"));
assert_eq!(it.next(), Some(&"c"));
assert_eq!(it.next(), None);

// 数组还能继续用
println!("{:?}", arr);  // ✅ 没问题

// 对比 into_iter()：它会消耗数组
let mut it2 = arr.into_iter();
assert_eq!(it2.next(), Some("a"));  // Some(&str) — 没有额外的 &
// println!("{:?}", arr);  // ❌ 编译错误，arr 已被消耗
```

**通俗理解**：`.iter()` 只"看一看"，不"拿走"。如果直接给 `"banana"` 本身，意味着把数组里的元素移走了，数组就被破坏。Rust 的所有权不允许这样做，所以它给你一张"借用证"（引用），你只能读，不能改变数组。

---

### Q: `capitalize_first` 的实现（iterators2.rs）

**需求**：`"hello"` → `"Hello"`，空字符串返回 `""`

**错误写法**：
```rust
fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase(),  // ❌ 返回值没被接住
    }
    String::from(chars)  // ❌ Chars迭代器不能转String，且和上面match冲突
}
```

**正确写法**：
```rust
fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().chain(chars).collect(),
    }
}
```

**思路拆解**：
| 步骤 | 操作 | 说明 |
|------|------|------|
| 1 | `input.chars()` | 得到字符迭代器 `Chars` |
| 2 | `chars.next()` | 取出第一个字符，迭代器前进一格 |
| 3 | `first.to_uppercase()` | 返回大写转换迭代器 `ToUppercase`（不是 String！） |
| 4 | `.chain(chars)` | 把剩余字符迭代器接在后面，形成一个大迭代器 |
| 5 | `.collect()` | 把整个迭代器收集成 `String` |

**关键概念**：
- `to_uppercase()` 返回的是**迭代器**而非 `String`，因为一个字符的大写可能变成多个字符（如德语 `ß` → `SS`）
- `.chain()` 拼接两个同类型迭代器，懒求值
- `.collect()` 是消费方法，利用 `FromIterator` trait 将迭代器收集为目标类型

---

### Q: `result_with_list` vs `list_of_results`（iterators3.rs）

**需求**：
- `result_with_list` → `Ok([1, 11, 1426, 3])` — 全成功打包成一个 Ok，任一失败则短路返回 Err
- `list_of_results` → `[Ok(1), Ok(11), Ok(1426), Ok(3)]` — 每个结果独立保留

**`divide` 函数实现**：
```rust
fn divide(a: i64, b: i64) -> Result<i64, DivisionError> {
    if b == 0 {
        return Err(DivisionError::DivideByZero);
    }
    if a == i64::MIN && b == -1 {
        return Err(DivisionError::IntegerOverflow);
    }
    if a % b != 0 {
        return Err(DivisionError::NotDivisible);
    }
    Ok(a / b)
}
```

**两个函数对比**：

| 函数 | 返回类型 | `.collect()` 行为 |
|------|----------|-------------------|
| `result_with_list` | `Result<Vec<i64>, DivisionError>` | 自动短路：全 Ok → `Ok(vec)`，遇 Err → 返回该 Err |
| `list_of_results` | `Vec<Result<i64, DivisionError>>` | 原样收集，每个 Result 独立保留 |

```rust
fn result_with_list() -> Result<Vec<i64>, DivisionError> {
    let numbers = [27, 297, 38502, 81];
    numbers.into_iter().map(|n| divide(n, 27)).collect()
}

fn list_of_results() -> Vec<Result<i64, DivisionError>> {
    let numbers = [27, 297, 38502, 81];
    numbers.into_iter().map(|n| divide(n, 27)).collect()
}
```

**核心原理**：Rust 为标准库的 `Result` 实现了 `FromIterator<Result<A, E>> for Result<V, E> where V: FromIterator<A>`。同一个 `.collect()` 调用，只因**目标类型不同**就产生不同行为：

```
.map(...).collect::<Result<Vec<i64>, _>>()  →  短路收集
.map(...).collect::<Vec<Result<i64, _>>>()  →  原样收集
```

这就是为什么 `Result` 类型的顺序是 `Result<Vec<i64>, DivisionError>`（成功在前，错误在后），不能写成 `Result<DivisionError, Vec<i64>>`。

**常见错误**：
```rust
// ❌ Result 参数顺序反了
fn result_with_list() -> Result<DivisionError, Vec<i32>> { ... }

// ❌ 多余的 filter，破坏了 collect 的自动短路
.map(|n| divide(n, 27)).filter(|r| r.is_ok()).collect()
// filter 后仍然是 Result 迭代器，collect 到 Vec 会变成 Vec<Result<_, _>>
```

---

### Q: 用迭代器实现阶乘（iterators4.rs）

**需求**：`factorial(n)` = `1 * 2 * 3 * ... * n`，不使用循环/递归/额外变量/early return。

```rust
fn factorial(num: u64) -> u64 {
    (1..=num).product()
}
```

**关键点**：

| `num` | `1..=num` | `.product()` | 结果 |
|-------|-----------|--------------|------|
| 0 | 空范围 `1..=0` | 1（乘法的单位元） | `0! = 1` ✅ |
| 1 | `1..=1` → `[1]` | 1 | `1! = 1` ✅ |
| 2 | `1..=2` → `[1, 2]` | 2 | `2! = 2` ✅ |
| 4 | `1..=4` → `[1,2,3,4]` | 24 | `4! = 24` ✅ |

**常用消费方法一览**：

| 方法 | 作用 | 空迭代器返回值 |
|------|------|----------------|
| `.sum()` | 求和 | `0` |
| `.product()` | 求积 | `1` |
| `.count()` | 计数 | `0` |
| `.collect()` | 收集到集合 | 空集合 |
| `.fold(init, f)` | 自定义折叠 | 返回 `init` |

都是单表达式，一条链解决，无需 for 循环。

---

### Q: `count_iterator` 的 `filter_map` vs `filter`（iterators5.rs）

**需求**：统计 HashMap 中值等于 `value` 的条目数量。

**错误代码及问题**：
```rust
map.iter().filter_map(|(_,v)| Some(*v == value)).count()
//                       总是 Some → 等于没过滤，count 了全部条目
```

**正确写法**：
```rust
fn count_iterator(map: &HashMap<String, Progress>, value: Progress) -> usize {
    map.values().filter(|v| *v == value).count()
}

fn count_collection_iterator(collection: &[HashMap<String, Progress>], value: Progress) -> usize {
    collection.iter().flat_map(|m| m.values()).filter(|v| *v == value).count()
}
```

**`filter` vs `filter_map` 对比**：

| 方法 | 闭包返回 | 行为 | 适用场景 |
|------|----------|------|----------|
| `filter(f)` | `bool` | `true` 保留原元素，`false` 丢弃 | 纯过滤 |
| `filter_map(f)` | `Option<T>` | `Some(x)` 保留 `x`，`None` 丢弃 | 过滤 + 转换 |

```rust
let nums = [1, 2, 3, 4, 5];

// filter：只判断条件
let evens: Vec<_> = nums.iter().filter(|x| *x % 2 == 0).collect();
// → [2, 4]

// filter_map：同时过滤和转换
let doubled_evens: Vec<_> = nums.iter().filter_map(|x| {
    if *x % 2 == 0 { Some(x * 2) } else { None }
}).collect();
// → [4, 8]
```

**`flat_map` 的作用**：把嵌套的迭代器"摊平"。`collection` 是 `&[HashMap<_, _>]`，`flat_map(|m| m.values())` 把每个 HashMap 的值迭代器串成一个平层迭代器。

**`iter()` / `values()` 的类型链条**：
```
&HashMap<K, V>  →  .values()  →  Iterator<Item = &V>
&HashMap<K, V>  →  .iter()    →  Iterator<Item = (&K, &V)>
```

所以 `map.values()` 直接拿到 `&Progress`，比 `map.iter()` 再解构元组更简洁。
