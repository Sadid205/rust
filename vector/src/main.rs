fn main() {
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    let third = &v[2];
    println!("The third element is {}", third);
}
