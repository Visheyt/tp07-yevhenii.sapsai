use std::char;
use std::env;

#[derive(Debug)]
struct Arg {
    double: bool,
    triple: bool,
    word: String,
}

fn main() {
    let arg = get_arg();
    letterscore(&arg.word);
    println!("Score = {}", score(arg));
}

fn get_arg() -> Arg {
    let mut arg = Arg {
        double: false,
        triple: false,
        word: String::new(),
    };

    let args: Vec<String> = env::args().skip(1).collect();

    for i in args {
        if &i[0..1] == "-" {
            if i == "-d" || i == "--double" {
                arg.double = true;
            } else if i == "-t" || i == "--triple" {
                arg.triple = true;
            } else if i == "-dt" || i == "-td" {
                arg.double = true;
                arg.triple = true;
            } else {
                arg.triple = false;
                arg.double = false;
            }
        } else {
            arg.word = i;
        }
    }
    if arg.word.is_empty() {
        eprintln!("The word to score is missing");
        std::process::exit(1)
    }
    arg
}

type Score = u8;

fn letterscore(word: &String) {
    for i in word.to_lowercase().chars() {
        println!("{}: {}", i, char_to_score(i));
    }
}

fn score(arg: Arg) -> Score {
    let mut k = 1;
    if arg.double == true && arg.triple == true {
        println!("x2\nx3");
        k = 2 * 3
    } else if arg.triple == true {
        println!("x3");
        k = 3
    } else if arg.double == true {
        println!("x2");
        k = 2
    }
    k * arg
        .word
        .chars()
        .flat_map(lowercase)
        .map(char_to_score)
        .fold(0, sum)
}

fn char_to_score(c: char) -> Score {
    match c {
        'a' => 1,
        'e' => 1,
        'i' => 1,
        'o' => 1,
        'u' => 1,
        'l' => 1,
        'n' => 1,
        'r' => 1,
        's' => 1,
        't' => 1,
        'd' => 2,
        'g' => 2,
        'b' => 3,
        'c' => 3,
        'm' => 3,
        'p' => 3,
        'f' => 4,
        'h' => 4,
        'v' => 4,
        'w' => 4,
        'y' => 4,
        'k' => 5,
        'j' => 8,
        'x' => 8,
        'q' => 10,
        'z' => 10,
        _ => 0,
    }
}

fn lowercase(e: char) -> char::ToLowercase {
    e.to_lowercase()
}

fn sum(a: u8, b: u8) -> u8 {
    a + b
}
