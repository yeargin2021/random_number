use rand::Rng;

fn main() {
    // Create a random number generator
    let mut rng = rand::thread_rng();

    // Generate a random number between 1 and 5 (inclusive)
    let random_number: u32 = rng.gen_range(1..=5);

    // Print the random number
    println!("Random number between 1 and 5: {}", random_number);
}
