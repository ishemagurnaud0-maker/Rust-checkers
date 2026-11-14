mod ownership;
mod borrow;
mod structs;
mod school_mis;
mod enums;
mod collections;
mod errors_handler;


fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let mut sum = add(5, 10);
    
    // Cloning the String to create a new instance

    if sum % 2 == 0 {
        println!("The sum is even.");
    } else {
        println!("The sum is odd.");
    }
    println!("The sum is: {sum}");

    let person: (&str, i32) = ("Alice", 30);
    println!("Person: {}, Age: {}", person.0, person.1);

    // loops
    for i in 1..=5 {
        println!("Number: {}", i);
    }

    loop {
        println!("This will print forever!");
        sum += 1;

        if sum > 20 {
            break;
        }
    }

    let scores: [i32; 5] = [1, 2, 3, 4, 5];

    for score in scores {
        println!("For every element looped through give out {score} BTC");
    }

    
    // ownership::owner_ship();
    // borrow::borrow();
    // structs::main();
    // school_mis::main();
    // collections::collections();
    // collections::mappings();
    // errors_handler::logger();
    // errors_handler::find_panic();
    // errors_handler::main();
   // errors_handler::calculate_grade(101);
   errors_handler::custom(0);
   
   let sum: Result<f32, &'static str> = enums::operate(enums::Operations::Add(10.0, 5.0));
   println!("Sum: {:?}", sum);
}




