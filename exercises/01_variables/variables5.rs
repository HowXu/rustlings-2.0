fn main() {
    let number = "T-H-R-E-E"; // Don't change this line
    println!("Spell a number: {number}");

    /*
    *
    * 在 Rust 中，你需要用 let 来 遮蔽(shadow) 变量：
    * let number = 3;
    * 这样就在 number = 3; 那行加上 let，创建一个新的同名变量，编译器错误就解决了。变量名不变，类型可以自由切换。
    *
    * */

    // TODO: Fix the compiler error by changing the line below without renaming the variable.
    let number = 3;
    println!("Number plus two is: {}", number + 2);
}
