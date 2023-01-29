fn main() {
    let x:i16 = 10;
    if x>5 {
        println!("X is greater than 5");
    } else {
        println!("X is smaller");
    }

    // match statement
    let num: i16 = 4;
    let num_word = match num {
        1 => "One",
        2 => "Two",
        3 => "Three",
        4 => "Four",
        _ => "Unknown"
    };
    println!("{}",num_word);
}