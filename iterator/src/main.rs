fn main() {
    #[derive(Debug)]
    struct Order {
        item: String,
        price: i32,
    }

    let numbers = vec![10, 20, 30];
    let mut num_iter = numbers.iter();
    println!("{:?}", num_iter.next()); // আউটপুট: Some(10)
    println!("{:?}", num_iter.next()); // আউটপুট: Some(20)
    println!("{:?}", num_iter.next()); // আউটপুট: Some(30)
    println!("{:?}", num_iter.next()); // আউটপুট: None (লিস্ট শেষ!)

    let total: i32 = numbers.iter().sum();
    println!("Total {}", total);

    let orders = vec![
        Order {
            item: String::from("Mouse"),
            price: 500,
        },
        Order {
            item: String::from("Keyboard"),
            price: 1500,
        },
        Order {
            item: String::from("Monitor"),
            price: 12000,
        },
    ];

    let min_price = 1000;

    let discounted_prices: Vec<i32> = orders
        .into_iter()
        .filter(|order| order.price >= min_price)
        .map(|order| order.price - 200)
        .collect();

    println!("Discountted price: {:?}", discounted_prices);
}
