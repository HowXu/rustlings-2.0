// Structs contain data, but can also have logic. In this exercise, we have
// defined the `Package` struct, and we want to test some logic attached to it.

#[derive(Debug)]
struct Package {
    sender_country: String,
    recipient_country: String,
    weight_in_grams: u32,
}

// impl 关键字 这个是 组合一个函数进去吧
impl Package {
    // Self关键字是返回Package
    // 不带引用self为第一参数的应该是静态函数 这里这个是工厂类型
    fn new(sender_country: String, recipient_country: String, weight_in_grams: u32) -> Self {
        if weight_in_grams < 10 {
            // This isn't how you should handle errors in Rust, but we will
            // learn about error handling later.
            panic!("Can't ship a package with weight below 10 grams");
        }
        // 返回值
        Self {
            sender_country,
            recipient_country,
            weight_in_grams,
        }
    }

    // TODO: Add the correct return type to the function signature.
    // 这种应该是对象方法
    fn is_international(&self) -> bool{
        // TODO: Read the tests that use this method to find out when a package
        // is considered international.
        !(self.recipient_country.eq(&self.sender_country))
    }

    // TODO: Add the correct return type to the function signature.
    fn get_fees(&self, cents_per_gram: u32) -> u32 {
        // TODO: Calculate the package's fees.
        cents_per_gram * self.weight_in_grams
    }
    // 用引用可以直接.取对象内容 这个应该是对Copy Trail深拷贝 对非Copy Trail浅拷贝 &是借用 下级拿到的也是借用 所有权没有变化 不会直接消失
    // 多数情况下你切记要用&借用 引起所有权变化会很麻烦
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn fail_creating_weightless_package() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Austria");

        Package::new(sender_country, recipient_country, 5);
    }

    #[test]
    fn create_international_package() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Russia");

        let package = Package::new(sender_country, recipient_country, 1200);

        assert!(package.is_international());
    }

    #[test]
    fn create_local_package() {
        let sender_country = String::from("Canada");
        let recipient_country = sender_country.clone();

        let package = Package::new(sender_country, recipient_country, 1200);

        assert!(!package.is_international());
    }

    #[test]
    fn calculate_transport_fees() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Spain");

        let cents_per_gram = 3;

        let package = Package::new(sender_country, recipient_country, 1500);

        assert_eq!(package.get_fees(cents_per_gram), 4500);
        assert_eq!(package.get_fees(cents_per_gram * 2), 9000);
    }
}
