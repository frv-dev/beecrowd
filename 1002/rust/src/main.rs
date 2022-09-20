use std::io;

fn main() {
    let mut input_radius: String = String::new();
    const PI: f64 = 3.14159;

    io::stdin()
        .read_line(&mut input_radius)
        .expect("Error on reading radius!");
    
    let radius: f64 = input_radius.trim()
        .parse::<f64>()
        .unwrap();
    
    println!("A={:.4}", PI * radius.powi(2));
}
