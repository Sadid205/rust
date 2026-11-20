fn main() {
    let discount_rate = 10;

    // let apply_discount = |price: i32| -> i32 { price - (price * discount_rate / 100) };
    fn apply_discount(price: i32) -> i32 {
        return price - (price * discount_rate / 100);
    }

    println!("100 টাকার পণ্যের দাম: {}", apply_discount(100));
    println!("500 টাকার পণ্যের দাম: {}", apply_discount(500));
}
