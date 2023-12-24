use std::fs::File;
use std::io::{self, BufRead};
fn read_from_file() -> io::Result<Vec<String>> {
    let file = File::open("./src/input.txt").expect("No file found");
    let reader = io::BufReader::new(file);

    let mut content: Vec<String> = Vec::new();

    for line in reader.lines(){
        content.push(line?)
    }

    Ok(content)

}
#[allow(non_snake_case)]
fn main() {
    let content = read_from_file().expect("No file found");
    let mut sum = 0;
    for line in content{
        let mut numbers : Vec<u32> = Vec::new();
        for char in line.chars(){
            if char.is_digit(10) {
               numbers.push(char.to_digit(10).unwrap() ) 
            }
        }
        sum += numbers[0] * 10 + numbers[numbers.len() - 1];
    }
    println!("Result is {sum}")
}