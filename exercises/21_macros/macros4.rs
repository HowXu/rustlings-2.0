// TODO: Fix the compiler error by adding one or two characters.
#[rustfmt::skip]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    }; // 这个匹配规则是要用;分开的
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    } // 注意这是取值语法 $val 为变量名 $必须 expr表示这是一个表达式 $val用来取值
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
