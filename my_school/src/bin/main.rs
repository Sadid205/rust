fn main () {

    my_school::class_10::take_exam();
    let s = my_school::class_10::new_student("Sadid");
    println!("Student Name: {}", s.name);
    // println!("Student Marks: {}", s.marks); // Error: marks is private
}