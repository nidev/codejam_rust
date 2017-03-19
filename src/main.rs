use std::io;
use std::str::FromStr;

type Number = i32;

fn solve() -> &'static str {
    "Solved"
}

fn main() {
    let cases = get_num_in_line().unwrap();

    // some problem solving...

    for casenum in 1..(cases + 1) {
        println!("Case {}: {}", casenum, solve());
    }
}

#[allow(dead_code)]
fn get_line() -> String {
    let mut buf = String::new();
    let _ = io::stdin().read_line(&mut buf);
    buf.trim_right().to_string()
}

#[allow(dead_code)]
fn get_words_in_line() -> Vec<String> {
    let text = get_line();
    let mut words: Vec<String> = Vec::new();

    for x in text.split_whitespace() {
        words.push(x.to_string());
    }

    words
}

#[allow(dead_code)]
fn get_nums_in_line() -> Vec<Number> {
    let mut buf = String::new();
    let mut numbers: Vec<Number> = Vec::new();

    match io::stdin().read_line(&mut buf) {
        Ok(_) => {
            let raw_input = buf.trim_right();
            let inputs = raw_input.split_whitespace();

            for x in inputs {
                numbers.push(i32::from_str(x).unwrap());
            }
        },
        Err(_) => {
        }
    }
    assert!(numbers.len() > 0, "Result is empty");

    numbers
}

#[allow(dead_code)]
fn get_num_in_line() -> Option<Number> {
    let mut buf = String::new();

    match io::stdin().read_line(&mut buf) {
        Ok(_) => {
            match i32::from_str(buf.trim_right()) {
                Ok(x) => {
                    return Some::<Number>(x);
                },
                Err(_) => {
                    println!("Error!")
                }
            }
        },
        Err(_) => {

        }
    }

    None::<Number>
}
