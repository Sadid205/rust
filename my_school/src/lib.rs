pub mod class_10 {

    pub fn take_exam() {
        println!("take_exam");
    }

    pub struct Student {
        pub name: String, // সবাই দেখতে পাবে
        marks: i32,       // কেউ দেখতে পাবে না (Private)
    }

    pub fn new_student(name: &str) -> Student {
        Student {
            name: String::from(name),
            marks: 100,
        }
    }
}


