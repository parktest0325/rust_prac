use std::io;

fn main() {
    println!("Enter your weight (kg): ");
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    println!("Input: {}", input);

    dbg!(&input);
    let weight: f32 = input.trim().parse().unwrap();
    dbg!(&weight);

    let mars_weight = calculate_weight_on_mars(weight);
    println!("Weight on Mars: {}kg", mars_weight);
}

fn calculate_weight_on_mars(_weight: f32) -> f32 {
    (_weight / 9.81) * 3.711
}
