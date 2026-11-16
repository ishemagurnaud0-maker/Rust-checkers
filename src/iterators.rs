
pub fn iterators() {

    let mut numbers = vec![20, 39, 38, 47, 84];
    
    for number in numbers.iter() {
        println!("{}", number);
    }

    for number in numbers.iter_mut() {
        *number += 5;

        println!("{}", number);
    }

    for number in numbers.into_iter() {
        println!("{}", number);
    }

    
}