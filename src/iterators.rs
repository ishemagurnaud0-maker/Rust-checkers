
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


//combining closures and iterators

//using .map()


pub fn use_map_fn() {
    let add = |mark: i32| mark + 5;

    let marks: Vec<i32> = [89, 48, 56, 44, 100];

    marks.iter().map(
        add()
    )
}
