# Modules

In this section we'll give you an introduction to Rust's module system.

## Further information

- [The Module System](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)

---

## 笔记：Rust 模块是什么？

### 通俗理解

模块（module）就是代码的"文件夹"——把相关代码组织在一起，形成命名空间。

### 三个核心关键字

| 关键字 | 作用 |
|--------|------|
| `mod` | 定义/声明一个模块 |
| `use` | 把模块路径引入当前作用域，简化访问 |
| `pub` | 让模块内的东西对外可见（默认一切都是私有的） |

### 基础用法

```rust
mod animal {
    fn secret() {}        // 默认私有，外部无法访问
    pub fn eat() {        // pub 才能被外部调用
        secret();
    }
}

fn main() {
    animal::eat();        // :: 路径分隔符
}
```

### 模块的文件组织方式

| 方式 | 写法 | 说明 |
|------|------|------|
| 内联 | `mod foo { ... }` | 模块内容直接写在同一个文件里 |
| 同目录文件 | `mod foo;` → `foo.rs` | 模块内容放在同名 `.rs` 文件 |
| 同目录文件夹 | `mod foo;` → `foo/mod.rs` | 模块内容放在同名文件夹的 `mod.rs` 里 |

### use 的常见写法

```rust
use std::collections::HashMap;      // 引入单个类型
use std::io::{self, Read, Write};   // 引入多个，self 指代 io 模块本身
use std::fs::*;                     // 引入所有公开项（谨慎使用）
```

### 可见性规则速记

- 不加 `pub` → 仅在当前模块及子模块可见
- `pub` → 外部可访问
- `pub(crate)` → 当前 crate 内可见
- `pub(super)` → 父模块可见
- `pub(in path)` → 指定路径内可见

### 与 Java 的类比

| Java | Rust | 作用 |
|------|------|------|
| `package com.example;` | `mod foo;` | 命名空间/组织代码 |
| `class Dog { ... }` | `struct Dog { ... }` + `impl Dog { ... }` | 数据 + 行为 |
| `import com.example.Dog;` | `use crate::foo::Dog;` | 引入路径 |

Rust 的 `mod` 更像 Java 的 `package`，只负责组织代码和控制可见性，不包含数据。数据和行为由 `struct` + `impl` 承担（这才对应 Java 的 `class`）。

### `use` 的可见性：默认私有

`use` 本身**不默认公开**——它只把名称引入当前模块作用域。想让外部也能通过本模块访问被引入的东西，需要加 `pub`：

```rust
mod a {
    mod b {
        pub fn hello() {}
    }
    use b::hello;      // 私有：a 内部能用，外部不能
    pub use b::hello;  // 重新导出：外部可以通过 a::hello 调用
}
```

| 写法 | 当前模块可见 | 外部可见 |
|------|:----------:|:------:|
| `use xxx;` | ✓ | ✗ |
| `pub use xxx;` | ✓ | ✓ |

这种 `pub use` 模式叫 **re-export（重新导出）**——把内部模块里的东西以当前模块的名义"转发"出去，方便外部调用。

---

## 笔记：Crate 是什么？

### 通俗理解

crate（包箱）是 Rust 的**编译单元**——编译器一次处理一个 crate，生成一个二进制文件或一个库。`

### crate 的两种形态

| 类型 | 入口文件 | 产物 | 场景 |
|------|----------|------|------|
| binary crate | `src/main.rs` | 可执行文件 (`.exe`) | 应用程序 |
| library crate | `src/lib.rs` | `.rlib` 库文件 | 给别人用的库 |

一个 Cargo 项目（一个 `Cargo.toml`）默认就是一个 crate，比如 `cargo new my_project` 创建出来的就是一个 binary crate。

### crate 与 package、module 的关系（三层体系）

| 层级 | 概念 | 说明 | 类比 |
|------|------|------|------|
| package | 包 | 一个 `Cargo.toml` 描述的项目，可包含多个 crate | Java 的 Maven/Gradle 项目 |
| crate | 包箱 | 编译单元，一个 package 至少有 1 个 crate | Java 的 JAR 包 |
| module | 模块 | crate 内部的代码组织单位，`mod` 定义 | Java 的 package |

```
package (Cargo.toml)
├── binary crate (src/main.rs)
│   ├── mod foo → foo.rs
│   │   ├── fn bar()
│   │   └── struct Baz
│   └── mod utils → utils/
│       └── mod.rs
└── library crate (src/lib.rs)  ← 可选，一个 package 最多一个
    └── ...
```

### 重要规则

1. 一个 package **至少**有一个 crate（默认是 binary）
2. 一个 package **最多**有一个 library crate
3. 一个 package 可以有**多个** binary crate（放在 `src/bin/` 目录下，每个文件一个）
4. `crate::` 是 crate 根路径的绝对路径引用

### `crate::` 路径示例

```rust
// src/main.rs (crate root)
mod animal {
    pub fn eat() {}
}

fn main() {
    crate::animal::eat();  // crate:: 从 crate 根开始寻址
}
```

### 外部 crate 的引入

在 `Cargo.toml` 添加依赖后，通过 `use` 引入：

```rust
// Cargo.toml
// [dependencies]
// rand = "0.8"

use rand::Rng;  // 使用外部 crate
```

### 对比速记

| 概念 | 你写的 | 作用 |
|------|--------|------|
| 定义模块 | `mod xxx;` | 把代码分层组织 |
| 定义 crate | `Cargo.toml` + `main.rs`/`lib.rs` | 定义一个编译单元 |
| 引入外部 crate | `Cargo.toml` 加依赖 + `use` | 使用别人的代码 |
| crate 根路径 | `crate::` | 从 crate 顶端绝对寻址 |
| 外部 crate 路径 | `rand::`（crate 名直接作为根） | 访问外部 crate |
