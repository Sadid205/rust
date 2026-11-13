// fn main() {
//     // println!("Hello, world!");

//     another_function(5);
// }


// fn another_function(x: i32){
//     println!("The value of x is: {}", x);
// }




// fn main() {
//     print_labeled_measurement(5,'h');
// }


// fn print_labeled_measurement(value: i32, unit_label: char) {
//     println!("The measurement is: {value}{unit_label}");
// }


// fn main() {
//     let y = (let y = 6); 
// }


fn main () {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    return x + 1;
}
