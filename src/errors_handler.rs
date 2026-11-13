

//Error Handling

// 1. Use special Enum Result<T, E> recoverable errors
// 2. Use panic! for unrecoverable errors




pub fn get_user(id: u32) -> Result<u32, String> {
    if id == 1 {
        Ok(id)
    }else {
        Err("Student not found");
    }
}



pub fn logger() {
    
    let first_id: u32 = 4;
    let second_id: u32 = 1;

    println!("{}", get_user(first_id));
    println!("{}", get_user(second_id));
}



// Use panic!


fn find_panic() {

    let scores = vec![0, 2, 4, 3, 5, 6];
    let any_elt = &scores.get(10);
    
    match any_elt {
        Some(elt) => {
            println!("{}", elt);
        }

        None => {
            panic!("array index is out of bounds.")
        }
    }
    

    


}