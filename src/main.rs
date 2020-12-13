mod day_four;
fn main() {
    match day_four::solve("input.txt") {
        Ok(result) => println!("Answer: {}", result),
        Err(e) => println!("Err: {}", e),
    }
}
