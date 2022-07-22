use std::io; // import the io module

fn main() {
    // Take input from the user
    println!("Enter a number: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // calculate the weight on Mars, trim the input and parse it to a float
    let mut weight : f32 = calc_mars_weight(input.trim().parse::<f32>().unwrap());
    println!("Your weight on Mars is: {}kg", weight);

}

fn calc_mars_weight(weight : f32) -> f32 {
    // calculate the weight on Mars using our acceleration of 9.81 m/s^2, multiplied by th acceleration of 3.711 m/s^2 to get the weight on Mars
    (weight / 9.81) * 3.711
}

