use std::io;

fn main() {
    let mut number_input: String = String::new();

    io::stdin()
        .read_line(&mut number_input)
        .expect("Error on reading number.");
    
    let number_1: f32 = number_input.trim()
        .parse::<f32>()
        .unwrap();
    
    number_input = String::new();

    io::stdin()
        .read_line(&mut number_input)
        .expect("Error on reading number.");
    
    let number_2: f32 = number_input.trim()
        .parse::<f32>()
        .unwrap();

    let product: f32 = number_1 * number_2;
    
    println!("PROD = {:.0}", product);
}
