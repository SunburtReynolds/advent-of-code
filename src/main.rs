mod day_two;
fn main() {
    match day_two::solve("input.txt") {
        Ok(result) => println!("Answer: {}", result),
        Err(e) => println!("Err: {}", e),
    }
}
