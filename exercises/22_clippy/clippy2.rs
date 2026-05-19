fn main() {
    let mut res = 42;
    let option = Some(12);
    // TODO: Fix the Clippy lint.
    // 这里只有一次循环 如果是While会直接退出的吧 只能是if的模式匹配
    if let Some(x) = option {
        res += x;
    }

    println!("{res}");
}
