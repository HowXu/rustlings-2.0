fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;
    // 这是深拷贝还是浅拷贝 是直接复制指针过去还是复制值过去
    // let mut vec = vec; 是 移动语义（move），不是深拷贝也不是浅拷贝。
    // 底层仅复制了栈上的 3 个字段（指针、长度、容量），堆上的数据没有复制。
    // 但原绑定 vec（参数）被编译器标记为失效，防止 double-free。

    vec.push(88);

    vec
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Make both vectors `vec0` and `vec1` accessible at the same time to
    // fix the compiler error in the test.
    #[test]
    fn move_semantics2() {
        let vec0 = vec![22, 44, 66];

        // rust 移动语义
        let vec1 = fill_vec(vec0.clone()); // now vec0 is 失效的 需要一个深拷贝

        assert_eq!(vec0, [22, 44, 66]);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}
