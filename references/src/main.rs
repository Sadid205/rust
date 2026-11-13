fn main() {
    let mut s1 = String::from("hello");

    calculate_length(&mut s1);

    println!("The new word is {s1}.");
}

fn calculate_length(s: &mut String)  {
    // s.len() 
    s.push_str(",world");
}
