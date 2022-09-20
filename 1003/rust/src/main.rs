use std::io;

fn main() {
    let mut number_input: String = String::new();
    
    io::stdin()
        .read_line(&mut number_input)
        .expect("Error on reading number.");
    
    let number_1: i8 = number_input.trim()
        .parse::<i8>()
        .unwrap();
    
    number_input = String::new();

    io::stdin()
        .read_line(&mut number_input)
        .expect("Error on reading number.");

    let number_2: i8 = number_input.trim()
        .parse::<i8>()
        .unwrap();
    
    println!("SOMA = {}", number_1 + number_2);
}
