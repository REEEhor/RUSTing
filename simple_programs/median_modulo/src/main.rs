use std::collections::HashMap;

fn main() {
    let numbers = vec![1, 2, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    match modus(&numbers) {
        Some(m) => println!("Modus is: {m}"),
        None => println!("List of values was empty"),
    }

    match median(&numbers) {
        Some(m) => println!("Median is: {m}"),
        None => println!("List of values was empty"),
    }
}

fn median(values: &Vec<i32>) -> Option<i32> {
    if values.is_empty() {
        return None;
    }
     
    let mut copy: Vec<i32> = values.clone();
    copy.sort();

    let middle = copy.len() / 2;

    return Some(copy[middle]);
}

fn modus(values: &Vec<i32>) -> Option<i32> {
    let mut modus: Option<i32> = None;
    let mut map: HashMap<i32, usize> = HashMap::new();
    let mut max_count = 0usize;

    for n in values {
        let count = map.entry(*n).or_insert(0);
        *count += 1;
        if *count > max_count {
            modus = Some(*n);
            max_count = *count;
        }
    }

    return modus;
}
