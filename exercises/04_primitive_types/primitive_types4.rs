fn main() {
    // You can optionally experiment here.
}


#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];

        // TODO: Get a slice called `nice_slice` out of the array `a` so that the test passes.
        // let nice_slice = ???
        // slice语法为&[初始索引..终点索引] 遵循左闭右开区间
        let nice_slice = &a[1..a.len() - 1];

        assert_eq!([2, 3, 4], nice_slice);
    }
}
