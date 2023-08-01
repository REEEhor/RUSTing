fn main() {
    println!();

    //////////////////////////////////////////////////////////////////////////////////
    // Using expressions    

    // using expression to set a value to a variable
    let _area = {
        const PI: f64 = 3.1415926535;
        let radius = 1.0;
        PI * 2.0 * radius
    };

    // using expression with loops to set a value to a variable
    let mut counter = 5;
    let mut result = 1;
    let five_factorial = loop {
        if counter <= 1 {
            break result; // "returns" value out of the loop
        }
        result *= counter;
        counter -= 1;
    };
    println!("5! is: {five_factorial}");

    // if can also be used to set a value to a variable
    let condition = true;
    let _number = if condition { 5 } else { 6 };

    println!();
    //////////////////////////////////////////////////////////////////////////////////
    // Dealing with for loops and arrays

    let arr = [10, 20, 33, 40, 50];
    for element in arr {
        println!("element has the value: {element}");
    }

    println!();
    //////////////////////////////////////////////////////////////////////////////////
    // For loops

    for num in 1..5 {
        println!("the number is: {num}");
    }
    println!();

    for num in (1..5).rev() {
        println!("the number is: {num}");
    }

    println!();
}
