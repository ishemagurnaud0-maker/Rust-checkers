
pub fn borrow() {

    let mut name1 = String::from("John");
    let name2 = &name1; // Borrowing name1 immutably

    display_name(name2); 
    
    modify_name(&mut name1); // Borrowing name1 mutably

    println!("Final Name: {}", name1);

}

fn display_name(name: &String) {
    println!("Name: {name}");
}

fn modify_name(name: &mut String) {
    name.push_str(" Doe");
    println!("Modified Name: {name}");
}