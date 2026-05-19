// TODO: Fix the compiler error by moving the whole definition of this macro.
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

// 宏是不能越过定义域阶段的 必须先声明才能用

fn main() {
    my_macro!();
}

