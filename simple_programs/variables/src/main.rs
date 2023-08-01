fn main() {
    let _kocka: char = '😻'; // wtf, you can store emoji

    ////////////////////////////////////////////////////////////
    println!("\"changing\" imutable values by shadowing");
    let num = 5;
    println!("The value of num is \"{num}\""); // prints 5
    let num = 6;
    println!("The value of num is \"{num}\""); // prints 6

    {
        let num = 7;
        println!("The value of num in the inner scope is \"{num}\""); // prints 7
    }

    // inner scope ends => shadowing %num with the value of "7" also ends
    println!("The value of num is \"{num}\""); // prints 6 again
    println!("_____________________________________________\n\n\n");
    ////////////////////////////////////////////////////////////
    ////////////////////////////////////////////////////////////

    ////////////////////////////////////////////////////////////
    ////////////////////////////////////////////////////////////
    println!("tuple type");
    let tup = (42, 69, 'g');
    // println!("{tup}"); // cannot print tuples

    // destructing
    let (_a, b, _c) = tup;
    println!("The value of the second value in the tuple (42, 69, 'g') is: \"{b}\"");
    // accessing using period
    println!(
        "The value of the second value in the tuple (42, 69, 'g') is: {}",
        tup.1
    );
    // so printing can be done with:
    println!(
        "Printing tuple manually: \"({},{},{})\"",
        tup.0, tup.1, tup.2
    );
    println!("Printing tuple using formating:{:?}", tup);

    // unit
    let _unit = ();
    println!("_____________________________________________\n\n\n");
    ////////////////////////////////////////////////////////////
    ////////////////////////////////////////////////////////////

    ////////////////////////////////////////////////////////////
    ////////////////////////////////////////////////////////////
    println!("arrays");

    // declaration with square brackets
    let _days_of_week = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
    const _NUMBERS: [i32; 2] = [3, 4];
    let arr = [3; 10];

    println!(
        "Elements of arr declared as \"let arr = [3;10];\" are:\n{:?}",
        arr
    );

    println!("_____________________________________________\n\n\n");
    ////////////////////////////////////////////////////////////
    ////////////////////////////////////////////////////////////
}
