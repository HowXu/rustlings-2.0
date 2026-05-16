// TODO: Fix the compiler error in the function without adding any new line.
fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(88);
    // 为什么我可以把参数声明为mut 然后传入一个unmut的参数它就变成mut了?
    // 关于 mut 参数： mut 是绑定（binding）的属性，不是类型的一部分。
    // vec0 是不可变绑定，但它被移动进 fill_vec 后，所有权转移了——
    // 函数内部可以用 mut 声明一个新的可变绑定来持有同一个值，并修改它。
    // 本质上是同一个 Vec，只是在不同作用域有不同的可变性声明。
    // 细说绑定和移动语义
    vec
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics3() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}
