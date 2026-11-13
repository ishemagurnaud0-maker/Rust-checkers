
use std::collections::HashMap;



// vectors


pub fn collections() {

    let ages = vec![12, 15, 28, 45, 34];

    println!("{:?}", ages);
    println!("{:?}", ages[3]);

    println!("{:?}", ages.get(4)); // returns type Option<T> => Some(T) or None

    let mut scores = vec![98, 100, 87, 90, 92, 97, 99];

    scores.push(90);
    scores.pop();

    let mut totalMarks: u32 = 0; // type u32 has the Copy trait 

    for score in &scores {
       totalMarks += score;
    }

   let average: u32 = average(totalMarks, scores.len().try_into().unwrap());

   println!("{}", average);

}

pub fn average(totalMarks: u32, numberOfModules: u32) -> u32 {
    totalMarks/numberOfModules
}


// HashMaps 

pub fn mappings() {
   
   let mut scores: HashMap<String, u32> = HashMap::new();

   scores.insert(String::from("John"), 98);
   scores.insert(String::from("Mary"), 100);
   scores.insert(String::from("James"), 96);


   println!("{:?}", scores.get("John"));
   println!("{:?}", scores.get("john"));

   scores.remove("Peter");
   scores.remove("John");


   for (name, _mark) in &scores {
      match &scores.get(name) {
        Some(mark) => { 
            println!("{} scored {}", name, mark);
        }
        None => {
            println!("The student does not exist");
        }
      }
   }
}
