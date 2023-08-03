fn main() {
    let num1 = 5;
    let _num2 = num1; // value is copied
    println!("Value of num1 is: {num1}");

    let str1 = String::from("Ahoj");
    let str2 = str1;
    // The println!() does not work since str1 is out of scope now
    // println!("Value of str1 is: {str1}");

    let str1 = str2.clone();
    // this works since we cloned the string
    println!("Value of str1 is: {str1}");
    // it is now also possible use the str2 now at the same time
    // (which you cannot without the use of '.clone()')
    println!("Value of str2 is: {str2}");

    ////////////////////////////////////////////////////////////////////////////////////////////////////////

    takes_ownership(str1);
    // we can no longer print str1 - it was *moved* into the function 'takes_ownership()'
    // println!("Value of str1 is: {str1}");

    just_makes_copy(num1);
    // using num1 is valid, since it was copied
    println!("Value of num1 is: {num1}");

    let mut str1 = str2;
    // this is ok, but annoying - when function needs a value, it also has to
    // return it if we don't want to lose ownership
    str1 = takes_ownership_and_gives_back(str1);
    println!("Value of str1 is: {str1}");

    ////////////////////////////////////////////////////////////////////////////////////////////////////////

    let len = calculates_length_by_borrowing_ownership_using_a_reference(&str1);
    // using borrowing, we can still use the str1 even after calling using it as a argument
    println!("Lenght of the string \"{str1}\" is {len}");

    // mutable references are also possible
    append_a_string_using_mutable_reference(&mut str1);
    println!("value of str1 is: {str1}");

    // references can also be variables
    let reference = &str1;
    calculates_length_by_borrowing_ownership_using_a_reference(reference);
    let mut_ref1 = &mut str1;
    // let mut_ref2 = &mut str1; // cannot have multiple mutable references at once if we want to use the first one of them
    println!("Value of str1 is (using a mut reference): {mut_ref1}");

    ////////////////////////////////////////////////////////////////////////////////////////////////////////
    // String Slices

    let mut text = String::from("Cau jak se mas?");
    let word = first_word(&text);
    // text.clear(); // invalid since first_word() function borrows a reference to text
    println!("The first word of \"{text}\" is \"{word}\"");
    text.clear();

    ////////////////////////////////////////////////////////////////////////////////////////////////////////
    // Borrowing

    let mut _vec = vec![1, 2, 3];
    let _num = &_vec[0];
    steal_ownership(_vec);
    // Cannot do this
    // println!("{_num}");
}

fn steal_ownership(_vec: Vec<i32>) {}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..index];
        }
    }

    return &s[..];
}

fn append_a_string_using_mutable_reference(string_reference: &mut String) {
    string_reference.push_str(", svete!");
}

fn calculates_length_by_borrowing_ownership_using_a_reference(string_reference: &String) -> usize {
    string_reference.len()
}

fn takes_ownership(some_string: String) {
    println!(
        "function took ownership over the string: \"{}\"",
        some_string
    );
}

fn just_makes_copy(some_integer: i32) {
    println!("function prints the copied value: \"{}\"", some_integer);
}

fn takes_ownership_and_gives_back(some_string: String) -> String {
    some_string
}
