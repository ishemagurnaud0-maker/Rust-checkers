

 #[derive(Debug)]
 pub enum StudentStatus {
    Active,
    Suspended,
    Graduated

 }



#[derive(Debug)] 
 pub enum Operations {
    Add(f32,f32),
    Subtract(f32,f32),
    Multiply(f32,f32),
    Divide(f32,f32),
}


pub fn operate(op: Operations) -> Result<f32, &'static str> {
    match op {
        Operations::Add(a,b) => Ok(a + b),
        Operations::Subtract(a,b) => Ok(a - b),
        Operations::Multiply(a,b) => Ok(a * b),
        Operations::Divide(a,b) => if b == 0.0 {
            println!("Error: Division by zero");
            Err("Division by zero is not allowed")
        } else { 
            Ok(a / b)
        },
    }



}
 

