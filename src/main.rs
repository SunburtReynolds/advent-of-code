mod day_three;
fn main() {
    match day_three::solve("input.txt") {
        Ok(result) => println!("Answer: {}", result),
        Err(e) => println!("Err: {}", e),
    }
}
