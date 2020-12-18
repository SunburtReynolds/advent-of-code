mod day_six;
fn main() {
    match day_six::solve("input.txt") {
        Ok(result) => println!("Answer: {}", result),
        Err(e) => println!("Err: {}", e),
    }
}
