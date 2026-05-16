# Enums

Rust allows you to define types called "enums" which enumerate possible values.
Enums are a feature in many languages, but their capabilities differ in each language. Rust's enums are most similar to algebraic data types in functional languages, such as F#, OCaml, and Haskell.
Useful in combination with enums is Rust's "pattern matching" facility, which makes it easy to run different code for different values of an enumeration.

## Further information

- [Enums](https://doc.rust-lang.org/book/ch06-00-enums.html)
- [Pattern syntax](https://doc.rust-lang.org/book/ch19-03-pattern-syntax.html)


|格式|	用途	|实现 trait	|必须 derive?
|---|---|---|---|
|{}	|用户友好输出|	Display	|手动实现
|{:?}	|调试输出	|Debug	|#[derive(Debug)] 自动生成
|{:#?}|	美化 Debug|	Debug	|同上