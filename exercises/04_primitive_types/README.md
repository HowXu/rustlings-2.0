# Primitive Types
原始类型 或者内置类型

Rust has a couple of basic types that are directly implemented into the
compiler. In this section, we'll go through the most important ones.

## Further information

- [Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html)
- [The Slice Type](https://doc.rust-lang.org/book/ch04-03-slices.html)

---

## 笔记：数组 `[T; N]` vs 切片 `[T]`

| 特性 | 数组 `[T; N]` | 切片 `[T]` |
|------|--------------|-----------|
| 大小 | 编译时确定，N 是类型一部分 | 动态大小类型（DST），运行时确定 |
| 可否作为值持有 | 可以，栈上分配 | 不能，只能通过引用 `&[T]` |
| 类型区分 | `[i32; 3]` ≠ `[i32; 5]` | 不同长度的切片同属 `[i32]` |

### 代码示例

```rust
let arr: [i32; 3] = [1, 2, 3];           // 数组
let slice: &[i32] = &arr;                 // 切片引用（完整数组）
let slice2: &[i32] = &arr[0..2];          // 切片引用（部分数组）

// 数组自动 Deref 为切片引用
fn sum(values: &[i32]) -> i32 {
    values.iter().sum()
}
println!("{}", sum(&arr));                // 传入 [i32; 3]，自动转为 &[i32]
println!("{}", sum(&slice2));             // 传入 &[i32]，直接匹配
```

### `&[T]` 是胖指针

`&[T]` 在内存中占 2 个 usize：一个指向数据起始地址，一个存储长度。而 `&[T; N]` 只是普通指针（1 个 usize），因为长度在类型里已隐含。

### 核心类比

- **数组** = 拥有一整栋房子
- **切片** = 拍了一张（可能只包含部分）房子的照片——能看到里面的内容，但不拥有它
