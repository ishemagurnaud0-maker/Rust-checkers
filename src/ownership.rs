
 pub fn owner_ship() {
  let name1 = String::from("John");
  let name2 = &name1;

  println!("Name1: {}", name1); // This will cause a compile-time error because name1 has been moved to name2
  println!("Name2: {}", name2);
  
}