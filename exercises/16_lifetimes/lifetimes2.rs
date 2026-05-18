// Don't change this function.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // TODO: Fix the compiler error by moving one line.

    let string1 = String::from("long string is long");
    let result;
    let string2 = String::from("xyz");
    // 显然生命周期是相对于作用域而言的 括号分级的作用域里的变量不可能和外层一个生命周期
    {
        
        result = longest(&string1, &string2);
    }
    println!("The longest string is '{result}'");
}
