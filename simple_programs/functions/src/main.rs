fn main() {
    // let x = five();
    // my_function(x);
    const X: i32 = five();
    my_function(X);
}

fn my_function(x: i32) {
    println!("my_function({x}) called!");
}

const fn five() -> i32 {
    5
}

fn _five() -> i32 {
    return 5;
}
