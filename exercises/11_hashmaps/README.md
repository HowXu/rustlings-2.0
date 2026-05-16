# Hashmaps

A *hash map* allows you to associate a value with a particular key.
You may also know this by the names [*unordered map* in C++](https://en.cppreference.com/w/cpp/container/unordered_map),
[*dictionary* in Python](https://docs.python.org/3/tutorial/datastructures.html#dictionaries) or an *associative array* in other languages.

This is the other data structure that we've been talking about before, when
talking about Vecs.

## Further information

- [Storing Keys with Associated Values in Hash Maps](https://doc.rust-lang.org/book/ch08-03-hash-maps.html)

---

## 笔记

### `#[derive(Hash, PartialEq, Eq, Debug)]` 的含义

`#[derive(...)]` 是 Rust 的**自动 trait 实现**机制，编译器帮你生成代码，不用手写。

| trait | 作用 | 为什么需要 |
|-------|------|-----------|
| `Hash` | 把值计算成一个哈希值（整数） | HashMap 通过哈希值快速定位 key |
| `PartialEq` | 允许 `==` 和 `!=` 比较 | 哈希冲突时需要判断 key 是否真相等 |
| `Eq` | 标记 `==` 满足自反性（a == a 永远为真） | HashMap key 的强制要求 |
| `Debug` | 允许 `{:?}` 格式化打印 | 方便调试（非 HashMap 必须） |

**HashMap 的 key 必须同时满足 `Hash` + `Eq`**（因为 `Eq` 要求先有 `PartialEq`，所以实际上要三个）。

```rust
// 没有 derive — 不能用作 HashMap key，编译报错
enum Fruit { Apple, Banana }
// map.insert(Fruit::Apple, 42);  // ❌ 编译错误

// 有 derive — 一切正常
#[derive(Hash, PartialEq, Eq, Debug)]
enum Fruit { Apple, Banana }
let mut map = HashMap::new();
map.insert(Fruit::Apple, 42);  // ✅
println!("{:?}", map);
```

### `try_insert` 的平替方案

`try_insert` 在 Rust 1.70 才稳定。更常用且兼容所有版本的替代是 **`entry` API**。

三种方式对比：

```rust
// 方式1: try_insert (1.70+)
basket.try_insert(Fruit::Banana, 3);

// 方式2: entry + or_insert (最惯用，推荐)
basket.entry(Fruit::Banana).or_insert(3);

// 方式3: 手动判断（啰嗦 + 两次哈希查找，不推荐）
if !basket.contains_key(&Fruit::Banana) {
    basket.insert(Fruit::Banana, 3);
}
```

`entry` 的优势：
- 只做**一次**哈希查找（方式3做了两次）
- `Entry` 枚举分 `Vacant`（不存在）和 `Occupied`（已存在）两种状态
- 链式 API 丰富：`or_insert` / `or_insert_with` / `and_modify` 等

本练习的典型解法：
```rust
for fruit in fruit_kinds {
    basket.entry(fruit).or_insert(1);
}
```

### `entry` 触发 `or_insert` 的条件

只有 key **不存在**（`Vacant` 状态）时才会触发 `or_insert`。key 已存在（`Occupied`）时，`or_insert` 被跳过，直接返回已有值的可变引用。

```rust
let mut map = HashMap::new();
map.insert("a", 1);

map.entry("a").or_insert(99);  // "a" 已存在，不动 → map["a"] 仍是 1
map.entry("b").or_insert(99);  // "b" 不存在，插入 → map["b"] 变成 99
```

### `and_modify` — 对已存在的值做修改

`and_modify` 只在 key **已存在**（`Occupied`）时触发，拿到 `&mut V` 让你修改原值。常与 `or_insert` 链式使用：

```rust
// 经典组合：计数
map.entry(key)
   .and_modify(|v| *v += 1)  // 已存在 → +1
   .or_insert(1);             // 不存在 → 插入 1

// 三种常见用法：
// 1. or_insert 返回 &mut，怎么改都行
let v = map.entry("a").or_insert(0);
*v += 10;  // 不管新旧，最终都 +10

// 2. 只改已存在的，不存在就跳过
map.entry("a").and_modify(|v| *v *= 2);

// 3. 不存在插入默认值，存在就修改
map.entry("a")
   .and_modify(|v| *v = v.to_uppercase())
   .or_insert("default".to_string());
```

执行逻辑：`Occupied` 走 `and_modify` 跳过 `or_insert`，`Vacant` 跳过 `and_modify` 走 `or_insert`。
