// closures




pub fn sum(a: i32, b: i32) {
    let add = |a: i32, b: i32| a + b;
    println!("Sum : {}", add(a,b));
}

pub fn hello() {
    let name = String:: from("Ishema");

    let greet = || {
        println!("Hello, {name}");
    };

    greet();
}



pub fn use_closures(score: f32) {

    let check_score = |score: f32| {
        if score >= 50.0 {
            println!("Pass");
        }else {
            println!("Fail");
        }
    };

    check_score(score);
}