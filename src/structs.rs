
// use crate::Enums;

// #[derive(Debug)]
// struct Student {
//     name: String,
//     age: u32,
//     marks: f32,
//     status: Enums::StudentStatus,
// }


// impl Student {
//     fn new(name: String, age: u32, marks: f32, status: Enums::StudentStatus) -> Student {
//         Student { name, age, marks, status }
//     }

//     fn display(&self) {
//         println!("Name: {}, Age: {}, Marks: {}, Status: {:?}", self.name, self.age, self.marks, self.status);
//     }

//     fn update_marks(&mut self, new_marks: f32) {
//         self.marks = new_marks;
//     }

//     fn update_age(&mut self, new_age: u32) {
//         self.age = new_age;
//     }

//     fn get_name(&self) -> &str {
//         &self.name
//     }

//      fn get_age(&self) -> u32 {
//         self.age
//      }

//      fn check_pass_mark(&self) -> bool {
//         self.marks >= 40.0
//      }
// }



// pub fn main() {
//     let mut student1 = Student::new(String::from("Alice"), 20, 85.5, Enums::StudentStatus::Active);
//     let mut student2 = Student::new(String::from("Bob"), 22, 35.0, Enums::StudentStatus::Suspended);

//     student1.display();
//     student2.display();

//     student1.update_marks(90.0);
//     student2.update_age(23);

//     println!("After updates:");
//     student1.display();
//     student2.display();

//     println!("{}'s age is: {}", student1.get_name(), student1.get_age());
//     println!("{}'s age is: {}", student2.get_name(), student2.get_age());

//     if student1.check_pass_mark() {
//         println!("{} has passed.", student1.get_name());
//     } else {
//         println!("{} has failed.", student1.get_name());
//     }

//     if student2.check_pass_mark() {
//         println!("{} has passed.", student2.get_name());
//     } else {
//         println!("{} has failed.", student2.get_name());
//     }

//     Student::display(&student1);
//     Student::display(&student2);


//     Student::update_marks(&mut student1, 95.56);
//     Student::update_age(&mut student1, 24);


//     match student1.status {
//         Enums::StudentStatus::Active => println!("{} is an active student.", student1.get_name()),
//         Enums::StudentStatus::Suspended => println!("{} is suspended.", student1.get_name()),
//         Enums::StudentStatus::Graduated => println!("{} has graduated.", student1.get_name()),
//         _ => println!("{} has an unknown status.", student1.get_name()),
//     }


//     if let Enums::StudentStatus::Active = student1.status {
//         println!("{} is an active student.", student1.get_name());
//     } else {
//         println!("{} is not an active student.", student1.get_name());
//     }
// }