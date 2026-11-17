// fn main() {
//     panic!("crash and burn");
// }
// use std::fs::File;
// fn main() {
//     match File::open("hello.txt") {
//         Ok(file) => println!("File opened successfully: {:?}", file),
//         Err(e) => println!("Error opening file: {:?}", e),
//     }
// }

// fn main() {
//     let r;              // r ঘোষণা করলাম, কিন্তু মান নেই
//     {
//         let x = 5;      // x শুধু এই ব্লকের ভেতরে বাঁচে
//         r = &x;         // r কে x-এর ঠিকানা দিলাম
//     }                   // ⬅️ এখানে x মরে গেল!
//     println!("{r}");    // ❌ r এখন মৃত x-কে দেখাচ্ছে
// }
