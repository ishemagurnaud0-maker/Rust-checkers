use std::fmt;

//Error Handling

// 1. Use special Enum Result<T, E> recoverable errors
// 2. Use panic! for unrecoverable errors




pub fn get_user(id: u32) -> Result<u32, String> {
    if id == 1 {
        Ok(id)
    }else {
        Err(String::from("Student not found."))
    }
}



pub fn logger() {
    
    let first_id: u32 = 4;
    let second_id: u32 = 1;

    println!("{:?}", get_user(first_id));
    println!("{:?}", get_user(second_id));
}



// Use panic!


pub fn find_panic() {

    let scores = vec![0, 2, 4, 3, 5, 6];
    let any_elt = &scores.get(10);
    
    match any_elt {
        Some(elt) => {
            println!("{}", elt);
        }

        None => {
            panic!("Array index is out of bounds.")
        }
    }
    
}


pub fn get_student_marks(id: u32) -> Option<f32> {

    if id == 100 {
        Some(83.5)
    } else {
        None
    }
}


pub fn main() {
    let result: Option<f32> = get_student_marks(101);

    match result {
        Some(value) => {
            println!("{}", value);
        }
        None => {
            println!("Student not found.");
        }
    }
}


pub fn get_marks(id: u32) -> Result<f32, String> {

    if id == 101 {
        Ok(80.0)
    }else if id == 102 {
        Ok(90.0)
    }else {
        Err(String::from("Student Not Found."))
    }
}


pub fn calculate_grade(id: u32) -> Result<String, String> {
    let marks: f32 = get_marks(id)?;

    if marks >= 80.0 {
        Ok(String::from("A"))
    }else {
        Ok(String::from("B"))
    }
}


//custom error handling 


pub enum StudentError {
    NotFound,
    InvalidId,
}


impl fmt::Display for StudentError {
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StudentError::NotFound => {
                write!(f, "Student Not Found")
            }
            StudentError::InvalidId => {
                write!(f, "Invalid student ID")
            }
        }
    }
}

pub fn get_username(id: u32) -> Result<String, StudentError> {
     if id == 0 {
        Err(StudentError::InvalidId)
     } else if id > 200 {
        Err(StudentError::NotFound)
     } else {
        Ok(String::from("Ishema"))
     }
        
}

pub fn custom(id: u32) {
    match get_username(id) {
        Ok(name) => {
            println!("{}", name);
        }
        Err(error) => {
            println!("{}", error)
        }
    }
}


