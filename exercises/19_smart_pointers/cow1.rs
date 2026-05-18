// This exercise explores the `Cow` (Clone-On-Write) smart pointer. It can
// enclose and provide immutable access to borrowed data and clone the data
// lazily when mutation or ownership is required. The type is designed to work
// with general borrowed data via the `Borrow` trait.

use std::borrow::Cow;

fn abs_all(input: &mut Cow<[i32]>) {
    for ind in 0..input.len() {
        let value = input[ind];
        if value < 0 {
            // Clones into a vector if not already owned.
            input.to_mut()[ind] = -value;
        }
    }
}

fn main() {
    // You can optionally experiment here.
}

/*
这个Cow比较灵性
Cow::from(&vec)是对的 Cow::from(vec) 也是对的 用法也是相同的 但是 所有权变化不一样
后者的所有权会给Cow 结束使用后就死掉了

除此之外 from(&vec)在不调用.to_mut的情况下一定是Cow::Borrowed(_) 调用.to_mut之后这个东西会转变成Cow::Owned(_)

关于Cow::Owned()和Cow::Borrowed(_) 二者表现出来是差不多的 只是在于 一个是写时无开销直接写 一个是读时无开销写时有

其实Owned是Borrowed的转化态 正常只需要Cow::from(&vec) 就可以了 底层是否to_mut没关系 vec所有权还在的

*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_mutation() {
        // Clone occurs because `input` needs to be mutated.
        let vec = vec![-1, 0, 1];
        let mut input = Cow::from(&vec);
        abs_all(&mut input);
        // 它传进去会受到to_mut的影响 所以需要Owned
        assert!(matches!(input, Cow::Owned(_)));
    }

    #[test]
    fn reference_no_mutation() {
        // No clone occurs because `input` doesn't need to be mutated.
        let vec = vec![0, 1, 2];
        let mut input = Cow::from(&vec);
        abs_all(&mut input);
        // TODO: Replace `todo!()` with `Cow::Owned(_)` or `Cow::Borrowed(_)`.
        // 因为这个值 它实际上 传进去不会受到to_mut的影响 所以只需要borrow就可以了
        assert!(matches!(input, Cow::Borrowed(_)));
    }

    #[test]
    fn owned_no_mutation() {
        // We can also pass `vec` without `&` so `Cow` owns it directly. In this
        // case, no mutation occurs (all numbers are already absolute) and thus
        // also no clone. But the result is still owned because it was never
        // borrowed or mutated.
        let vec = vec![0, 1, 2];
        let mut input = Cow::from(vec);
        abs_all(&mut input);
        // TODO: Replace `todo!()` with `Cow::Owned(_)` or `Cow::Borrowed(_)`.
        assert!(matches!(input, Cow::Owned(_)));
    }

    #[test]
    fn owned_mutation() {
        // Of course this is also the case if a mutation does occur (not all
        // numbers are absolute). In this case, the call to `to_mut()` in the
        // `abs_all` function returns a reference to the same data as before.
        let vec = vec![-1, 0, 1];
        let mut input = Cow::from(vec);
        abs_all(&mut input);
        // TODO: Replace `todo!()` with `Cow::Owned(_)` or `Cow::Borrowed(_)`.
        assert!(matches!(input, Cow::Owned(_)));
    }
}
