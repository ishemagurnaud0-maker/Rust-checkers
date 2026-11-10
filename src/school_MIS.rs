#[derive(Debug)]
struct Pupil{
    name: String,
    age: u32,
    status: Status
}

#[derive(Debug)]
enum Status {
    Active,
    Suspended,
    Graduated
}


impl Pupil {

    fn new(name: String, age: u32, status: Status) -> Pupil {
        Pupil{name, age, status}
    }

    fn display(&self) {
        println!("Name: {}, Age: {}, Status: {:?}", self.name, self.age, self.status);
    }

    fn update_age(&mut self, new_age: u32) {
        self.age = new_age;
    }

    fn update_status(&mut self, new_status: Status) {
        self.status = new_status;
    }

    fn check_status(&self) {
         

         match &self.status {
            Status::Active => {
                println!("{} is an active student.", self.name);
            }

            Status::Suspended => {
                println!("{} was put under suspension.", self.name);
            }

            Status::Graduated => {
                println!("{} has already graduated.", self.name);
            }
         }
    }
}


pub fn main() {

    let mut student1 = Pupil::new(String::from("Joe"), 23, Status::Suspended);
    student1.check_status();

    student1.display();

    student1.update_age(24);
    student1.update_status(Status::Active);
    student1.check_status();

}