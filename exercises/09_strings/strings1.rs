// TODO: Fix the compiler error without changing the function signature.
fn current_favorite_color() -> String {
    String::from("blue")
    // 这个是String对象的创建方式 根java有点像反正不是很正经
}

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {answer}");
}
