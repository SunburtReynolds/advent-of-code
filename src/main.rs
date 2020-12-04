mod day_one;
fn main() {
    match day_one::solve("input.txt") {
        Ok(result) => println!("Answer: {}", result),
        Err(e) => println!("Err: {}", e),
    }
}
