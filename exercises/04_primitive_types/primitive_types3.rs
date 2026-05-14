fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    // let a = ???
    let a:[isize;100] = [100;100];
    // 很怪异的语法 数据类型为[数据类型;容量] 批量命名为[数据;数量]

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
