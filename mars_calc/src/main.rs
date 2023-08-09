use std::io;

fn main() {
    let mut input = String::new();
    println!("Please enter your weight (kg): ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let weight: f32 = input.trim().parse()
        .expect("Please type a number!");
    println!("My weight on mars is: {:.2}kg", calculate_weight_on_mars(weight));
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    weight / 9.81 * 3.711
}