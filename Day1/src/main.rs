use std::collections::HashMap;
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
#[allow(dead_code)]
fn calculateFirstPart(content: &Vec<String>) -> u32 {
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
    sum
}

#[allow(non_snake_case)]
fn calculateSecondPart(content: &Vec<String>) -> u32 {
    let mut sum = 0;
    let possibilites = HashMap::from([
        ("1",1),
        ("2",2),
        ("3",3),
        ("4",4),
        ("5",5),
        ("6",6),
        ("7",7),
        ("8",8),
        ("9",9),
        ("one",1),
        ("two",2),
        ("three",3),
        ("four",4),
        ("five",5),
        ("six",6),
        ("seven",7),
        ("eight",8),
        ("nine",9)
    ]);
    for line in content{
        let mut numbers : Vec<u32> = Vec::new();
        println!("{line}");
        for i in 0..line.len(){
            for(key,value) in possibilites.iter(){
                if line[i..].starts_with(key) {
                    println!("{}",*value);
                    numbers.push(Some(*value).unwrap())
                }
            }
        }
        println!("SUM {} {} ",numbers[0], numbers[numbers.len() - 1]);
        sum += numbers[0] * 10 + numbers[numbers.len() - 1];
    }
    sum
}
#[allow(non_snake_case)]
fn main() {
    let content = read_from_file().expect("No file found");
    // let sum = calculateFirstPart(&content);
    let sum = calculateSecondPart(&content);
    println!("Result is {sum}")
}