// AsRef and AsMut allow for cheap reference-to-reference conversions. Read more
// about them at https://doc.rust-lang.org/std/convert/trait.AsRef.html and
// https://doc.rust-lang.org/std/convert/trait.AsMut.html, respectively.

// Obtain the number of bytes (not characters) in the given argument
// (`.len()` returns the number of bytes in a string).
// TODO: Add the `AsRef` trait appropriately as a trait bound.
fn byte_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().len()
}

// Obtain the number of characters (not bytes) in the given argument.
// TODO: Add the `AsRef` trait appropriately as a trait bound.
fn char_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().chars().count()
}

// Squares a number using `as_mut()`.
// TODO: Add the appropriate trait bound.
fn num_sq<T: AsMut<u32>>(arg: &mut T) {
    // 为了防止悬垂指针的问题 这里必须是引用mut Rust的引用规则是同一时刻只能有一个&mut 所以必须把带mut的传进来
    /*
    因为要返回 &mut T，前提是你能独占访问内部数据。&self 只给了只读权，无法保证没有别人也在看同一块内存——那就不能安全地借出可变引用。
     */
    // TODO: Implement the function body.
    let n = arg.as_mut(); // 外层是Box<>的引用 as_mut直接穿透了 先拿Box的mut 再穿透拿到u32的mut
    (*n) = (*n) * (*n);
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn different_counts_using_string() {
        let s = String::from("Café au lait");
        assert_ne!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn same_counts_using_string() {
        let s = String::from("Cafe au lait");
        assert_eq!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn mut_box() {
        let mut num: Box<u32> = Box::new(3);
        // 这里再返回又是多层装箱拿到Box指针了
        num_sq(&mut num);
        assert_eq!(*num, 9);
    }
}
