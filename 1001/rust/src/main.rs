use std::io;

fn main() {
    let mut input_data = String::new();
    
    io::stdin()
        .read_line(&mut input_data)
        .expect("Error!");
    
    let number_1: i8 = input_data.trim()
        .parse::<i8>()
        .unwrap();

    input_data = String::new();

    io::stdin()
        .read_line(&mut input_data)
        .expect("Error!");

    let number_2: i8 = input_data.trim()
        .parse::<i8>()
        .unwrap();
    
    println!("X = {}", number_1 + number_2);
}
