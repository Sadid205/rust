// use my_school::take_exam;

use my_school::class_10::section_a::attend_class;
use my_school::class_10::*;
fn main() {
    take_exam();
    attend_class();
    // let s = class_10::new_student("Sadid");
    // println!("Student Name: {}", s.name);
    // println!("Student Marks: {}", s.marks); // Error: marks is private
}
