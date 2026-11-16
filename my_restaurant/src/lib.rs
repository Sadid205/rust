pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("add_to_waitlist");
        }
    }

    pub mod serving {
        pub fn take_order() {
            println!("take_order");
        }
    }
}

fn deliver_order() {
    println!("deliver_order");
}


pub mod back_of_house {
    pub fn fix_incorrect_order(){
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {
        println!("cook_order");
    }
}


pub mod kitchen {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}
